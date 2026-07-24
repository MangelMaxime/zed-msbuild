use std::fs;

use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

use crate::language_servers::util;

const REPO: &str = "tintoy/msbuild-project-tools-server";
const ASSET_NAME: &str = "language-server.zip";
const SERVER_DLL: &str = "MSBuildProjectTools.LanguageServer.Host.dll";
const DOTNET_HINT: &str = "MSBuild Project Tools requires the .NET SDK (8.0+) on PATH. Install it \
from https://dotnet.microsoft.com/download, or set `lsp.msbuild-project-tools.binary.path` to a \
working launcher.";

pub struct MsbuildProjectTools {
    cached_dll_path: Option<String>,
}

impl MsbuildProjectTools {
    pub const LANGUAGE_SERVER_ID: &'static str = "msbuild-project-tools";

    pub fn new() -> Self {
        Self {
            cached_dll_path: None,
        }
    }

    pub fn language_server_cmd(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary_settings = LspSettings::for_worktree(Self::LANGUAGE_SERVER_ID, worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings.as_ref().and_then(|b| b.arguments.clone());

        if let Some(path) = binary_settings.and_then(|b| b.path) {
            return Ok(zed::Command {
                command: path,
                args: binary_args.unwrap_or_default(),
                env: Default::default(),
            });
        }

        if let Some(ref dll_path) = self.cached_dll_path {
            if fs::metadata(dll_path).is_ok_and(|stat| stat.is_file()) {
                return Self::dotnet_exec(worktree, dll_path, binary_args);
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == ASSET_NAME)
            .ok_or_else(|| format!("no asset found matching {ASSET_NAME:?}"))?;

        let version_dir = format!("{}-{}", Self::LANGUAGE_SERVER_ID, release.version);
        let dll_path = format!("{version_dir}/{SERVER_DLL}");

        if !fs::metadata(&dll_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            util::remove_outdated_versions(Self::LANGUAGE_SERVER_ID, &version_dir)?;
        }

        let dll_path = util::absolute_path(&dll_path)?;
        let command = Self::dotnet_exec(worktree, &dll_path, binary_args)?;
        self.cached_dll_path = Some(dll_path);
        Ok(command)
    }

    fn dotnet_exec(
        worktree: &zed::Worktree,
        dll_path: &str,
        user_args: Option<Vec<String>>,
    ) -> Result<zed::Command> {
        let dotnet = worktree
            .which("dotnet")
            .ok_or_else(|| DOTNET_HINT.to_string())?;

        let mut args = vec![dll_path.to_string()];
        if let Some(user) = user_args {
            args.extend(user);
        }

        // Microsoft.Build.Locator (used by the server to find an MSBuild install) shells out
        // to the dotnet muxer, so point it at the same `dotnet` we resolved above.
        let dotnet_host_path = fs::canonicalize(&dotnet)
            .ok()
            .and_then(|path| path.to_str().map(str::to_string))
            .unwrap_or_else(|| dotnet.clone());

        Ok(zed::Command {
            command: dotnet,
            args,
            env: vec![("DOTNET_HOST_PATH".to_string(), dotnet_host_path)],
        })
    }

    pub fn configuration_options(
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree(Self::LANGUAGE_SERVER_ID, worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings);
        Ok(settings)
    }
}
