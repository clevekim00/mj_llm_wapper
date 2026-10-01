use crate::{
    catalog::{load_catalog, recommend},
    device,
    domain::ModelRecommendation,
    runtime::{ModelRuntime, OllamaRuntime, RuntimeEvent},
};
use futures_util::StreamExt;
use std::{collections::HashSet, error::Error, io::Write};

const GIB: u64 = 1024 * 1024 * 1024;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Serve,
    Recommend {
        json: bool,
    },
    Install {
        model: String,
        yes: bool,
        force: bool,
    },
    AutoInstall {
        yes: bool,
    },
    Help,
}

pub fn parse<I>(args: I) -> Result<Command, String>
where
    I: IntoIterator<Item = String>,
{
    let args: Vec<String> = args.into_iter().collect();
    let Some(command) = args.first().map(String::as_str) else {
        return Ok(Command::Serve);
    };
    match command {
        "serve" => no_extra(&args).map(|_| Command::Serve),
        "recommend" => {
            reject_unknown_flags(&args[1..], &["--json"])?;
            Ok(Command::Recommend {
                json: args.iter().any(|value| value == "--json"),
            })
        }
        "install" => {
            reject_unknown_flags(&args[1..], &["--yes", "-y", "--force"])?;
            let model = args[1..]
                .iter()
                .find(|value| !value.starts_with('-'))
                .cloned()
                .ok_or_else(|| "install에는 모델 ID 또는 runtime tag가 필요합니다.".to_owned())?;
            Ok(Command::Install {
                model,
                yes: has_yes(&args),
                force: args.iter().any(|value| value == "--force"),
            })
        }
        "auto-install" => {
            reject_unknown_flags(&args[1..], &["--yes", "-y"])?;
            Ok(Command::AutoInstall {
                yes: has_yes(&args),
            })
        }
        "help" | "--help" | "-h" => Ok(Command::Help),
        value => Err(format!("알 수 없는 명령: {value}")),
    }
}

fn no_extra(args: &[String]) -> Result<(), String> {
    if args.len() == 1 {
        Ok(())
    } else {
        Err(format!("{} 명령에는 추가 인수가 없습니다.", args[0]))
    }
}

fn has_yes(args: &[String]) -> bool {
    args.iter()
        .any(|value| matches!(value.as_str(), "--yes" | "-y"))
}

fn reject_unknown_flags(args: &[String], allowed: &[&str]) -> Result<(), String> {
    if let Some(flag) = args
        .iter()
        .find(|value| value.starts_with('-') && !allowed.contains(&value.as_str()))
    {
        Err(format!("지원하지 않는 옵션: {flag}"))
    } else {
        Ok(())
    }
}

pub async fn run(command: Command, runtime_url: String) -> Result<(), Box<dyn Error>> {
    match command {
        Command::Serve => unreachable!("serve is handled by main"),
        Command::Help => {
            print_help();
            Ok(())
        }
        Command::Recommend { json } => recommend_command(runtime_url, json).await,
        Command::Install { model, yes, force } => {
            install_command(runtime_url, &model, yes, force).await
        }
        Command::AutoInstall { yes } => auto_install_command(runtime_url, yes).await,
    }
}

async fn recommendations(
    runtime: &OllamaRuntime,
) -> Result<Vec<ModelRecommendation>, Box<dyn Error>> {
    let status = runtime.status().await;
    let installed: HashSet<String> = status.installed_models.into_iter().collect();
    // Recommendation remains useful when Ollama is stopped. Runtime availability is
    // reported separately and must not hide the hardware fit calculation.
    let profile = device::detect(true);
    Ok(recommend(&load_catalog()?, &profile, &installed))
}

async fn recommend_command(runtime_url: String, json: bool) -> Result<(), Box<dyn Error>> {
    let runtime = OllamaRuntime::new(runtime_url);
    let status = runtime.status().await;
    let profile = device::detect(status.reachable);
    let rows = recommendations(&runtime).await?;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "device": profile,
                "runtime": {
                    "kind": status.kind,
                    "endpoint": status.endpoint_id,
                    "reachable": status.reachable
                },
                "recommendations": rows
            }))?
        );
        return Ok(());
    }

    println!(
        "장치: {} / {} CPUs / {}",
        profile.os,
        profile.logical_cpus,
        memory_label(profile.total_memory_bytes)
    );
    println!(
        "Ollama: {} ({})\n",
        if status.reachable {
            "연결됨"
        } else {
            "연결 안 됨 — 설치 전 실행 필요"
        },
        status.endpoint_id
    );
    println!(
        "{:<4} {:<28} {:<17} {:>9}  상태",
        "순위", "모델", "ID", "예상 RAM"
    );
    for (index, row) in rows.iter().enumerate() {
        println!(
            "{:<4} {:<28} {:<17} {:>9}  {}{}",
            index + 1,
            row.artifact.display_name,
            row.artifact.id,
            bytes(row.artifact.estimated_peak_ram_bytes),
            row.fit,
            if row.installed { " · 설치됨" } else { "" }
        );
    }
    println!("\n자동 설치: cargo run -- auto-install");
    Ok(())
}

async fn install_command(
    runtime_url: String,
    requested: &str,
    yes: bool,
    force: bool,
) -> Result<(), Box<dyn Error>> {
    let runtime = OllamaRuntime::new(runtime_url);
    let rows = recommendations(&runtime).await?;
    let row = resolve_model(&rows, requested)
        .ok_or_else(|| format!("카탈로그에서 모델을 찾을 수 없습니다: {requested}"))?;
    install_recommendation(&runtime, row, yes, force).await
}

async fn auto_install_command(runtime_url: String, yes: bool) -> Result<(), Box<dyn Error>> {
    let runtime = OllamaRuntime::new(runtime_url);
    let rows = recommendations(&runtime).await?;
    let row = choose_auto_install(&rows)
        .ok_or("설치 가능한 미설치 모델이 없습니다. recommend 결과를 확인하세요.")?;
    println!("자동 선택: {} — {}", row.artifact.display_name, row.reason);
    install_recommendation(&runtime, row, yes, false).await
}

async fn install_recommendation(
    runtime: &OllamaRuntime,
    row: &ModelRecommendation,
    yes: bool,
    force: bool,
) -> Result<(), Box<dyn Error>> {
    if row.installed {
        println!("이미 설치되어 있습니다: {}", row.artifact.runtime_model);
        return Ok(());
    }
    if row.fit == "blocked" && !force {
        return Err(format!(
            "{}은(는) 이 장치의 안전 메모리 한도를 초과합니다. 강제로 진행하려면 --force를 추가하세요.",
            row.artifact.display_name
        )
        .into());
    }
    let status = runtime.status().await;
    if !status.reachable {
        return Err(format!(
            "Ollama에 연결할 수 없습니다: {}. 먼저 `ollama serve`를 실행하세요.",
            status.endpoint_id
        )
        .into());
    }

    println!(
        "모델: {} ({})",
        row.artifact.display_name, row.artifact.runtime_model
    );
    println!(
        "다운로드: 약 {} / 예상 peak RAM: {}",
        bytes(row.artifact.download_bytes),
        bytes(row.artifact.estimated_peak_ram_bytes)
    );
    println!("장치 적합성: {} — {}", row.fit, row.reason);
    if !yes && !confirm("다운로드하고 설치할까요? [y/N] ")? {
        println!("취소했습니다.");
        return Ok(());
    }

    let mut stream = runtime.install(&row.artifact.runtime_model).await?;
    while let Some(event) = stream.next().await {
        match event? {
            RuntimeEvent::Progress(value) => print_progress(&value),
            RuntimeEvent::Done => break,
            _ => {}
        }
    }
    println!("\n설치 완료: {}", row.artifact.runtime_model);
    Ok(())
}

fn resolve_model<'a>(
    rows: &'a [ModelRecommendation],
    requested: &str,
) -> Option<&'a ModelRecommendation> {
    rows.iter()
        .find(|row| row.artifact.id == requested || row.artifact.runtime_model == requested)
}

fn choose_auto_install(rows: &[ModelRecommendation]) -> Option<&ModelRecommendation> {
    rows.iter()
        .find(|row| !row.installed && matches!(row.fit.as_str(), "recommended" | "possible"))
}

fn confirm(prompt: &str) -> Result<bool, std::io::Error> {
    print!("{prompt}");
    std::io::stdout().flush()?;
    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer)?;
    Ok(matches!(
        answer.trim().to_ascii_lowercase().as_str(),
        "y" | "yes"
    ))
}

fn print_progress(value: &serde_json::Value) {
    let status = value
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("진행 중");
    let completed = value.get("completed").and_then(serde_json::Value::as_u64);
    let total = value.get("total").and_then(serde_json::Value::as_u64);
    match (completed, total) {
        (Some(done), Some(total)) if total > 0 => {
            let percent = done.saturating_mul(100) / total;
            print!(
                "\r{status}: {percent:>3}% ({}/{})",
                bytes(done),
                bytes(total)
            );
        }
        _ => print!("\r{status}"),
    }
    let _ = std::io::stdout().flush();
}

fn memory_label(value: Option<u64>) -> String {
    value.map(bytes).unwrap_or_else(|| "확인 불가".into())
}

fn bytes(value: u64) -> String {
    if value >= GIB {
        format!("{:.1} GiB", value as f64 / GIB as f64)
    } else {
        format!("{:.1} MiB", value as f64 / (1024 * 1024) as f64)
    }
}

pub fn print_help() {
    println!(
        "MJ Local LLM Hub\n\n\
         사용법:\n  \
         mj-local-llm-hub serve\n  \
         mj-local-llm-hub recommend [--json]\n  \
         mj-local-llm-hub install <모델 ID|runtime tag> [--yes] [--force]\n  \
         mj-local-llm-hub auto-install [--yes]\n\n\
         인수 없이 실행하면 serve와 같습니다. --yes는 설치 확인을 생략합니다."
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ModelArtifact;

    fn row(id: &str, runtime_model: &str, fit: &str, installed: bool) -> ModelRecommendation {
        ModelRecommendation {
            artifact: ModelArtifact {
                id: id.into(),
                family: "test".into(),
                display_name: id.into(),
                runtime_model: runtime_model.into(),
                parameter_label: "1B".into(),
                download_bytes: 1,
                estimated_peak_ram_bytes: 1,
                context_tokens: 1,
                capabilities: vec![],
                platforms: vec![],
                lifecycle: "test".into(),
                source_url: "https://example.com".into(),
            },
            fit: fit.into(),
            fit_score: 1,
            reason: "test".into(),
            installed,
        }
    }

    #[test]
    fn parses_cli_commands() {
        assert_eq!(parse(Vec::<String>::new()).unwrap(), Command::Serve);
        assert_eq!(
            parse(["recommend".into(), "--json".into()]).unwrap(),
            Command::Recommend { json: true }
        );
        assert_eq!(
            parse(["install".into(), "qwen3:0.6b".into(), "-y".into()]).unwrap(),
            Command::Install {
                model: "qwen3:0.6b".into(),
                yes: true,
                force: false
            }
        );
    }

    #[test]
    fn resolves_id_and_runtime_tag() {
        let rows = vec![row("small", "vendor:small", "recommended", false)];
        assert!(resolve_model(&rows, "small").is_some());
        assert!(resolve_model(&rows, "vendor:small").is_some());
        assert!(resolve_model(&rows, "missing").is_none());
    }

    #[test]
    fn auto_install_skips_unsafe_and_installed_models() {
        let rows = vec![
            row("installed", "a:1", "recommended", true),
            row("blocked", "b:1", "blocked", false),
            row("possible", "c:1", "possible", false),
        ];
        assert_eq!(choose_auto_install(&rows).unwrap().artifact.id, "possible");
    }
}
