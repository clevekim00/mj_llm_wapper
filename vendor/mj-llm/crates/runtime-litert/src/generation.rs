use crate::RuntimeError;
use mj_llm_core::pin::VerifiedGenerationModel;
use std::path::Path;

pub struct GenerationEngine {
    #[cfg(feature = "native-macos")]
    inner: crate::native::NativeGenerator,
}
impl GenerationEngine {
    /// Consumes the owner after all synchronous generation calls have returned.
    pub fn unload(self) {}

    pub fn load(model: &VerifiedGenerationModel, cache: &Path) -> Result<Self, RuntimeError> {
        #[cfg(feature = "native-macos")]
        {
            Ok(Self {
                inner: crate::native::NativeGenerator::load(model, cache)?,
            })
        }
        #[cfg(not(feature = "native-macos"))]
        {
            let _ = (model, cache);
            Err(RuntimeError::Unavailable)
        }
    }
    /// Hub extension: bounded role-preserving history and per-request output cap.
    pub fn generate_messages(
        &mut self,
        messages: &[serde_json::Value],
        max_tokens: u32,
    ) -> Result<serde_json::Value, RuntimeError> {
        if messages.is_empty()
            || !(1..=32).contains(&max_tokens)
            || messages
                .iter()
                .map(|m| m["content"].as_str().map_or(usize::MAX / 1024, str::len))
                .sum::<usize>()
                > 1024
            || messages.len() > 32
            || messages.iter().any(|m| {
                m["content"].as_str().is_none_or(|s| s.trim().is_empty())
                    || !matches!(m["role"].as_str(), Some("system" | "user" | "assistant"))
            })
            || messages.last().and_then(|m| m["role"].as_str()) != Some("user")
            || messages.iter().skip(1).any(|m| m["role"] == "system")
        {
            return Err(RuntimeError::InvalidInput);
        }
        #[cfg(feature = "native-macos")]
        {
            self.inner.generate_messages(messages, max_tokens)
        }
        #[cfg(not(feature = "native-macos"))]
        {
            Err(RuntimeError::Unavailable)
        }
    }
    pub fn generate(&mut self, prompt: &str) -> Result<serde_json::Value, RuntimeError> {
        self.generate_messages(&[serde_json::json!({"role":"user","content":prompt})], 32)
    }
}
