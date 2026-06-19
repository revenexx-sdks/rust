use serde::{Deserialize, Serialize};

/// Template Runtime
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemplateRuntime {
    /// The build command used to build the deployment.
    #[serde(rename = "commands", default)]
    pub commands: String,
    /// The entrypoint file used to execute the deployment.
    #[serde(rename = "entrypoint", default)]
    pub entrypoint: String,
    /// Runtime Name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Path to function in VCS (Version Control System) repository
    #[serde(rename = "providerRootDirectory", default)]
    pub provider_root_directory: String,
}
