use serde::{Deserialize, Serialize};

/// Framework
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Framework {
    /// List of supported adapters.
    #[serde(rename = "adapters", default)]
    pub adapters: Vec<crate::models::FrameworkAdapter>,
    /// Default runtime version.
    #[serde(rename = "buildRuntime", default)]
    pub build_runtime: String,
    /// Framework key.
    #[serde(rename = "key", default)]
    pub key: String,
    /// Framework Name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// List of supported runtime versions.
    #[serde(rename = "runtimes", default)]
    pub runtimes: Vec<String>,
}
