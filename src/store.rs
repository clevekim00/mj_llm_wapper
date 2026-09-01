use crate::domain::Conversation;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{fs, sync::RwLock};

#[derive(Default, Serialize, Deserialize)]
struct StoreData {
    conversations: BTreeMap<String, Conversation>,
}

pub struct Store {
    path: PathBuf,
    data: RwLock<StoreData>,
}

impl Store {
    pub async fn open(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let data = match fs::read(&path).await {
            Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => StoreData::default(),
            Err(error) => return Err(error),
        };
        Ok(Self {
            path,
            data: RwLock::new(data),
        })
    }

    pub async fn conversations(&self) -> Vec<Conversation> {
        let mut values: Vec<_> = self
            .data
            .read()
            .await
            .conversations
            .values()
            .cloned()
            .collect();
        values.sort_by_key(|item| std::cmp::Reverse(item.updated_at_epoch_seconds));
        values
    }

    pub async fn save_conversation(&self, conversation: Conversation) -> std::io::Result<()> {
        let bytes = {
            let mut data = self.data.write().await;
            data.conversations
                .insert(conversation.id.clone(), conversation);
            serde_json::to_vec_pretty(&*data).expect("serializable store")
        };
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let temporary = self.path.with_extension("tmp");
        fs::write(&temporary, bytes).await?;
        fs::rename(temporary, &self.path).await
    }

    pub async fn delete_conversation(&self, id: &str) -> std::io::Result<bool> {
        let (removed, bytes) = {
            let mut data = self.data.write().await;
            let removed = data.conversations.remove(id).is_some();
            let bytes = serde_json::to_vec_pretty(&*data).expect("serializable store");
            (removed, bytes)
        };
        if removed {
            if let Some(parent) = self.path.parent() {
                fs::create_dir_all(parent).await?;
            }
            fs::write(&self.path, bytes).await?;
        }
        Ok(removed)
    }
}

pub fn now_epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn conversation_round_trip() {
        let directory = tempfile::tempdir().unwrap();
        let store = Store::open(directory.path().join("state.json"))
            .await
            .unwrap();
        store
            .save_conversation(Conversation {
                id: "one".into(),
                title: "테스트".into(),
                model: "qwen3".into(),
                messages: vec![],
                updated_at_epoch_seconds: 1,
            })
            .await
            .unwrap();
        assert_eq!(store.conversations().await.len(), 1);
        assert!(store.delete_conversation("one").await.unwrap());
    }
}
