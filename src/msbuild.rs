mod language_servers;

use zed_extension_api::{self as zed, Result};

use crate::language_servers::MsbuildProjectTools;

struct MsbuildExtension {
    msbuild_project_tools: Option<MsbuildProjectTools>,
}

impl zed::Extension for MsbuildExtension {
    fn new() -> Self {
        Self {
            msbuild_project_tools: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            MsbuildProjectTools::LANGUAGE_SERVER_ID => {
                let server = self
                    .msbuild_project_tools
                    .get_or_insert_with(MsbuildProjectTools::new);
                server.language_server_cmd(language_server_id, worktree)
            }
            language_server_id => Err(format!("unknown language server: {language_server_id}")),
        }
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        match language_server_id.as_ref() {
            MsbuildProjectTools::LANGUAGE_SERVER_ID => {
                MsbuildProjectTools::configuration_options(worktree)
            }
            _ => Ok(None),
        }
    }
}

zed::register_extension!(MsbuildExtension);
