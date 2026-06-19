use serde::{Deserialize, Serialize};

/// Framework Adapter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FrameworkAdapter {
    /// Default command to build site into output directory.
    #[serde(rename = "buildCommand", default)]
    pub build_command: String,
    /// Name of fallback file to use instead of 404 page. If null, Appwrite 404
    /// page will be displayed.
    #[serde(rename = "fallbackFile", default)]
    pub fallback_file: String,
    /// Default command to download dependencies.
    #[serde(rename = "installCommand", default)]
    pub install_command: String,
    /// Adapter key.
    #[serde(rename = "key", default)]
    pub key: String,
    /// Default output directory of build.
    #[serde(rename = "outputDirectory", default)]
    pub output_directory: String,
}
