use std::fs;
use zed::{CodeLabel, CodeLabelSpan, LanguageServerId};
use zed_extension_api::{self as zed, Result};

struct VExtension {
	current_version: String,
	cached_binary_path: Option<String>
}

fn try_local_install<T>(err: T, worktree: &zed::Worktree) -> Result<String, T> {
	if let Some(path) = worktree.which("v-analyzer") {
		return Ok(path);
	}
	return Err(err);
}

fn language_server_binary_path_no_fallback(
	selff: &mut VExtension,
	language_server_id: &LanguageServerId,
	worktree: &zed::Worktree,
) -> Result<String> {
	if let Some(cache) = selff.cached_binary_path.clone() {
		if let Some(local) = worktree.which("v-analyzer") {
			if local != cache && fs::metadata(&cache).map_or(false, |stat| stat.is_file()) {
				return Ok(cache);
			}
		} else {
			return Ok(cache);
		}
	}

	let (platform, arch) = zed::current_platform();
	zed::set_language_server_installation_status(
		&language_server_id,
		&zed::LanguageServerInstallationStatus::CheckingForUpdate,
	);

	let asset_name = format!(
		"v-analyzer-{os}-{arch}{extension}",
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
		extension = match platform {
			zed::Os::Windows => ".exe",
			_ => ""
		},
	);
	
	let release = zed::latest_github_release(
		"lv37/v-analyzer",
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


	if selff.current_version != asset.download_url || !fs::metadata(&asset_name).map_or(false, |stat| stat.is_file()) {
		zed::set_language_server_installation_status(
			&language_server_id,
			&zed::LanguageServerInstallationStatus::Downloading,
		);

		zed::download_file(
			&asset.download_url,
			&asset_name,
			zed::DownloadedFileType::Uncompressed,
		)
		.map_err(|e| format!("failed to download file: {e}"))?;

		zed::make_file_executable(&asset_name)?;

		let entries =
			fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
		for entry in entries {
			let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
			if entry.file_name().to_str() != Some(&asset_name) {
				fs::remove_dir_all(&entry.path()).ok();
			}
		}
	}
	selff.cached_binary_path = Some(asset_name.clone());
	selff.current_version = release.version;
	Ok(asset_name)
}

impl VExtension {
	fn language_server_binary_path(
		&mut self,
		language_server_id: &LanguageServerId,
		worktree: &zed::Worktree,
	) -> Result<String> {
		return language_server_binary_path_no_fallback(self, language_server_id, worktree)
			.or_else(|a| try_local_install(a, worktree));
	}
}

impl zed::Extension for VExtension {
	fn new() -> Self {
		Self {
			cached_binary_path: None,
			current_version: "".to_string()
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
