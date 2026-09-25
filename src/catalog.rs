use crate::domain::{DeviceProfile, ModelArtifact, ModelRecommendation};
use std::collections::HashSet;

pub fn load_catalog() -> Result<Vec<ModelArtifact>, serde_json::Error> {
    serde_json::from_str(include_str!("../catalog/models.json"))
}

pub fn recommend(
    catalog: &[ModelArtifact],
    device: &DeviceProfile,
    installed: &HashSet<String>,
) -> Vec<ModelRecommendation> {
    let available = device.total_memory_bytes.unwrap_or(4 * 1024_u64.pow(3));
    let mut rows: Vec<_> = catalog
        .iter()
        .cloned()
        .map(|artifact| {
            let ratio = artifact.estimated_peak_ram_bytes as f64 / available as f64;
            let (fit, score, reason) = if !device.runtime_reachable {
                (
                    "runtime-required",
                    20,
                    "로컬 Ollama 런타임을 시작해야 설치와 대화를 사용할 수 있습니다.",
                )
            } else if ratio <= 0.55 {
                (
                    "recommended",
                    95,
                    "메모리 안전 여유를 포함해 장시간 실행에 적합합니다.",
                )
            } else if ratio <= 0.78 {
                (
                    "possible",
                    70,
                    "실행 가능하지만 긴 context나 다른 앱 사용 시 메모리 압력을 확인해야 합니다.",
                )
            } else if ratio <= 0.95 {
                (
                    "not-recommended",
                    40,
                    "적재 가능성이 있으나 swap 또는 OOM 위험이 높습니다.",
                )
            } else {
                (
                    "blocked",
                    0,
                    "예상 peak RAM이 장치의 안전 한도를 초과합니다.",
                )
            };
            ModelRecommendation {
                installed: installed.contains(&artifact.runtime_model),
                artifact,
                fit: fit.into(),
                fit_score: score,
                reason: reason.into(),
            }
        })
        .collect();
    rows.sort_by(|a, b| {
        b.fit_score
            .cmp(&a.fit_score)
            .then_with(|| a.artifact.download_bytes.cmp(&b.artifact.download_bytes))
    });
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn low_memory_blocks_large_model() {
        let device = DeviceProfile {
            os: "linux".into(),
            architecture: "x86_64".into(),
            logical_cpus: 4,
            total_memory_bytes: Some(4 * 1024_u64.pow(3)),
            runtime_reachable: true,
        };
        let result = recommend(&load_catalog().unwrap(), &device, &HashSet::new());
        assert!(
            result
                .iter()
                .any(|row| row.artifact.family == "Mistral Small 3" && row.fit == "blocked")
        );
        assert_eq!(result.first().unwrap().artifact.family, "Qwen3");
    }

    #[test]
    fn catalog_contains_official_gpt_oss_ollama_variants() {
        let catalog = load_catalog().unwrap();
        let twenty = catalog
            .iter()
            .find(|model| model.runtime_model == "gpt-oss:20b")
            .expect("gpt-oss 20b catalog entry");
        let one_twenty = catalog
            .iter()
            .find(|model| model.runtime_model == "gpt-oss:120b")
            .expect("gpt-oss 120b catalog entry");

        assert_eq!(twenty.context_tokens, 131_072);
        assert!(twenty.capabilities.iter().any(|value| value == "text-only"));
        assert!(twenty.capabilities.iter().any(|value| value == "tools"));
        assert!(one_twenty.estimated_peak_ram_bytes > twenty.estimated_peak_ram_bytes);
        assert!(
            !twenty
                .platforms
                .iter()
                .any(|value| value == "mobile-candidate")
        );
    }

    #[test]
    fn sixteen_gib_device_does_not_recommend_gpt_oss() {
        let device = DeviceProfile {
            os: "macos".into(),
            architecture: "aarch64".into(),
            logical_cpus: 8,
            total_memory_bytes: Some(16 * 1024_u64.pow(3)),
            runtime_reachable: true,
        };
        let result = recommend(&load_catalog().unwrap(), &device, &HashSet::new());

        assert!(result.iter().any(|row| {
            row.artifact.runtime_model == "gpt-oss:20b"
                && matches!(row.fit.as_str(), "not-recommended" | "blocked")
        }));
        assert!(
            result.iter().any(|row| {
                row.artifact.runtime_model == "gpt-oss:120b" && row.fit == "blocked"
            })
        );
    }
}
