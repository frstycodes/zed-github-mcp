use std::env;

use serde::Deserialize;
use zed_extension_api::{self as zed, Extension, serde_json, settings::ContextServerSettings};

const PACKAGE_NAME: &str = "@modelcontextprotocol/server-github";
const PACKAGE_VERSION: &str = "2025.3.19";
const SERVER_PATH: &str = "node_modules/@modelcontextprotocol/server-github/dist/index.js";
const TOKEN_KEY: &str = "GITHUB_PERSONAL_ACCESS_TOKEN";

#[derive(Debug, Deserialize)]
struct GithubMCPSettings {
    github_personal_access_token: String,
}

struct GithubMCP;
impl Extension for GithubMCP {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _server_id: &zed_extension_api::ContextServerId,
        project: &zed_extension_api::Project,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }
        let settings = ContextServerSettings::for_project("github-context-server", project)?;

        let settings: GithubMCPSettings = match settings.settings {
            Some(s) => {
                serde_json::from_value(s).map_err(|e| format!("Failed to parse settings: {}", e))
            }
            None => {
                return Err("Missing 'github_personal_access_token'".to_string());
            }
        }?;

        let server_path = env::current_dir()
            .unwrap()
            .join(SERVER_PATH)
            .to_string_lossy()
            .to_string();

        Ok(zed::Command {
            command: "node".to_string(),
            args: vec![server_path],
            env: vec![(TOKEN_KEY.into(), settings.github_personal_access_token)],
        })
    }
zed::register_extension!(GithubMCP);
