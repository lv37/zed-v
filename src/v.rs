use std::fs;
use zed::{CodeLabel, CodeLabelSpan, LanguageServerId};
use zed_extension_api::{self as zed, Result};

struct VExtension {
	cached_binary_path: Option<String>,
}

impl VExtension {
	fn language_server_binary_path(
		&mut self,
		language_server_id: &LanguageServerId,
		worktree: &zed::Worktree,
	) -> Result<String> {
		if let Some(path) = worktree.which("v-analyzer") {
			return Ok(path);
		}

		if let Some(path) = &self.cached_binary_path {
			if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
				return Ok(path.clone());
			}
		}

		let (platform, arch) = zed::current_platform();
		zed::set_language_server_installation_status(
			&language_server_id,
			&zed::LanguageServerInstallationStatus::CheckingForUpdate,
		);

		let asset_name = format!(
			"v-analyzer-{os}-{arch}.zip",
			arch = match arch {
				zed::Architecture::Aarch64 => "arm64",
				zed::Architecture::X86 => "x86",
				zed::Architecture::X8664 => "x86_64",
			},
			os = match platform {
				zed::Os::Mac => "darwin",
				zed::Os::Linux => "linux",
				zed::Os::Windows => "windows",
			},
		);
		let version_dir = asset_name.clone();
		let binary_path = format!("{version_dir}/v-analyzer");

		if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
			zed::set_language_server_installation_status(
				&language_server_id,
				&zed::LanguageServerInstallationStatus::Downloading,
			);
			let release = zed::latest_github_release(
				"vlang/v-analyzer",
				zed::GithubReleaseOptions {
					require_assets: true,
					pre_release: false,
				},
			)?;

			let asset = release
			.assets
			.iter()
			.find(|asset| asset.name == asset_name)
			.ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

			zed::download_file(
				&asset.download_url,
				&version_dir,
				zed::DownloadedFileType::Zip,
			)
			.map_err(|e| format!("failed to download file: {e}"))?;

			let entries =
				fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
			for entry in entries {
				let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
				if entry.file_name().to_str() != Some(&version_dir) {
					fs::remove_dir_all(&entry.path()).ok();
				}
			}
		}
		self.cached_binary_path = Some(binary_path.clone());
		Ok(binary_path)
	}
}

impl zed::Extension for VExtension {
	fn new() -> Self {
		Self {
			cached_binary_path: None,
		}
	}

	fn language_server_command(
		&mut self,
		language_server_id: &LanguageServerId,
		worktree: &zed::Worktree,
	) -> Result<zed::Command> {
		Ok(zed::Command {
			command: self.language_server_binary_path(language_server_id, worktree)?,
			args: vec![],
			env: Default::default(),
		})
	}

	fn label_for_completion(
		&self,
		_language_server_id: &LanguageServerId,
		completion: zed::lsp::Completion,
	) -> Option<zed::CodeLabel> {
		let (label, start_idx) = match (completion.kind, completion.detail) {
			(_, None) => (completion.label, 0),
			(Some(zed::lsp::CompletionKind::Function), Some(a)) => (a, 3),
			(Some(zed::lsp::CompletionKind::Method), Some(a)) => {
				let pure = after_first(&a, ')').unwrap_or(a)[1..].to_string();
				(format!("fn {}", pure), 3)
			},
			(_, Some(a)) => (format!("{} {}", completion.label, a), 0),
		};

		Some(CodeLabel {
			spans: vec![CodeLabelSpan::code_range(start_idx..label.len())],
			filter_range: (0..label.len() - start_idx).into(),
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
