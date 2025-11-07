use std::env;
use std::fs;
use std::path::PathBuf;
use zed::{CodeLabel, CodeLabelSpan, LanguageServerId};
use zed_extension_api::{self as zed, Result};

struct VExtension {
    current_version: String,
    cached_binary_path: Option<String>,
}

// ✅ 平台自动检测 + fallback 逻辑
fn try_local_install<T>(err: T, worktree: &zed::Worktree) -> Result<String, T> {
    // 1️⃣ 查 PATH
    if let Some(path) = worktree.which("vls") {
        println!("Using vls from PATH: {}", path);
        return Ok(path);
    }

    // 2️⃣ fallback 到 ~/.vmodules/vls/vls 或 Windows 版本
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());

    // ⚙️ 根据系统类型选择可执行文件名
    #[cfg(target_os = "windows")]
    let exe_name = "vls.exe";
    #[cfg(not(target_os = "windows"))]
    let exe_name = "vls";

    let mut local_path = PathBuf::from(&home);
    local_path.push(".vmodules");
    local_path.push("vls");
    local_path.push(exe_name);

    if fs::metadata(&local_path).map_or(false, |m| m.is_file()) {
        println!("Using vls from local path: {}", local_path.display());
        return Ok(local_path.display().to_string());
    }

    // 3️⃣ 全部失败
    Err(err)
}

// 原始逻辑（保持不变）
fn language_server_binary_path_no_fallback(
    selff: &mut VExtension,
    _language_server_id: &LanguageServerId,
    _worktree: &zed::Worktree,
) -> Result<String> {
    if let Some(cache) = selff.cached_binary_path.clone() {
        if fs::metadata(&cache).map_or(false, |stat| stat.is_file()) {
            println!("Using cached vls path: {}", &cache);
            return Ok(cache);
        }
    }

    Err("vls not found".to_string())
}

impl VExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        language_server_binary_path_no_fallback(self, language_server_id, worktree)
            .or_else(|a| try_local_install(a, worktree))
            .map(|path| {
                self.cached_binary_path = Some(path.clone());
                path
            })
    }
}

impl zed::Extension for VExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
            current_version: "".to_string(),
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = self.language_server_binary_path(language_server_id, worktree)?;
        println!("Starting language server: {}", &path);
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
