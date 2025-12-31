use zed_extension_api::{self as zed, serde_json, settings::LspSettings, LanguageServerId, Result};

struct HareExtension;

#[derive(Clone)]
struct HareLspBinary {
    path: String,
    args: Option<Vec<String>>,
    environment: Option<Vec<(String, String)>>,
}

impl HareExtension {
    fn language_server_binary(
        &mut self,
        _: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<HareLspBinary> {
        let (platform, _) = zed::current_platform();
        let environment = match platform {
            zed::Os::Mac | zed::Os::Linux => Some(worktree.shell_env()),
            zed::Os::Windows => None,
        };

        if let Ok(lsp_settings) = LspSettings::for_worktree("hare-lsp", worktree) {
            if let Some(binary) = lsp_settings.binary {
                if let Some(path) = binary.path {
                    return Ok(HareLspBinary {
                        path: path.clone(),
                        args: binary.arguments,
                        environment,
                    });
                }
            }
        }

        if let Some(path) = worktree.which("hare-lsp") {
            return Ok(HareLspBinary {
                path,
                args: Some(vec!["-S".to_string()]),
                environment,
            });
        }

        Err("hare-lsp not found".to_string())
    }
}

impl zed::Extension for HareExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let hare_lsp_binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: hare_lsp_binary.path,
            args: hare_lsp_binary.args.unwrap_or_default(),
            env: hare_lsp_binary.environment.unwrap_or_default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("hare-lsp", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options)
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(HareExtension);
