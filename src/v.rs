use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use zed::{CodeLabel, CodeLabelSpan, LanguageServerId};
use zed_extension_api::{self as zed, Result};

struct VExtension {
    current_version: String,
    cached_binary_path: Option<String>,
}

// ---------------------------------------------
// ✅ 尝试从 PATH 或 ~/.vmodules/vls/vls 查找
// ---------------------------------------------
fn try_local_install<T>(err: T, worktree: &zed::Worktree) -> Result<String, T> {
    // ① 从 PATH 查找
    if let Some(path) = worktree.which("vls") {
        println!("Using system-installed vls: {}", &path);
        return Ok(path);
    }

    // ② 从 ~/.vmodules/vls/vls 查找
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    let mut local_path = PathBuf::from(home);
    local_path.push(".vmodules");
    local_path.push("vls");
    local_path.push(if cfg!(target_os = "windows") {
        "vls.exe"
    } else {
        "vls"
    });

    if fs::metadata(&local_path).map_or(false, |m| m.is_file()) {
        println!("Using vls from local path: {}", local_path.display());
        return Ok(local_path.display().to_string());
    }

    Err(err)
}

// ---------------------------------------------
// ✅ 若未找到，则执行 `v install vls` 自动安装
// ---------------------------------------------
fn ensure_vls_installed(language_server_id: &LanguageServerId) -> Result<String> {
    zed::set_language_server_installation_status(
        language_server_id,
        &zed::LanguageServerInstallationStatus::Downloading,
    );

    println!("Installing vls via: v install vls ...");

    let status = Command::new("v")
        .args(["install", "vls"])
        .status()
        .map_err(|e| format!("failed to spawn 'v install vls': {e}"))?;

    if !status.success() {
        return Err("`v install vls` failed".into());
    }

    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    let mut path = PathBuf::from(home);
    path.push(".vmodules");
    path.push("vls");
    path.push(if cfg!(target_os = "windows") {
        "vls.exe"
    } else {
        "vls"
    });

    if fs::metadata(&path).map_or(false, |m| m.is_file()) {
        zed::make_file_executable(&path.display().to_string())?;
        println!("vls successfully installed at: {}", path.display());
        return Ok(path.display().to_string());
    }

    Err("vls binary not found after installation".into())
}

// ---------------------------------------------
// ✅ 综合逻辑：缓存 + 自动安装 + fallback
// ---------------------------------------------
impl VExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(cache) = self.cached_binary_path.clone() {
            if fs::metadata(&cache).map_or(false, |stat| stat.is_file()) {
                return Ok(cache);
            }
        }

        try_local_install("vls not found".to_string(), worktree)
            .or_else(|_| ensure_vls_installed(language_server_id))
            .map(|path| {
                self.cached_binary_path = Some(path.clone());
                path
            })
    }
}

// ---------------------------------------------
// ✅ Zed 扩展实现
// ---------------------------------------------
impl zed::Extension for VExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
            current_version: String::new(),
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = self.language_server_binary_path(language_server_id, worktree)?;
        println!("Starting VLS from: {}", &path);
        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<CodeLabel> {
        let (label, start_idx) = match (completion.kind, completion.detail) {
            (_, None) => (completion.label, 0),
            (Some(zed::lsp::CompletionKind::Function), Some(a)) => (a, 3),
            (Some(zed::lsp::CompletionKind::Method), Some(a)) => {
                let pure = after_first(&a, ')').unwrap_or(a)[1..].to_string();
                (format!("fn {}", pure), 3)
            }
            (_, Some(a)) => (format!("{} {}", completion.label, a), 0),
        };

        Some(CodeLabel {
            spans: vec![CodeLabelSpan::code_range(start_idx..label.len())],
            filter_range: (0..(label.len() - start_idx)).into(),
            code: label,
        })
    }
}

fn after_first(in_string: &str, delim: char) -> Option<String> {
    let mut splitter = in_string.splitn(2, delim);
    splitter.next()?;
    Some(splitter.next()?.to_string())
}

zed::register_extension!(VExtension);
