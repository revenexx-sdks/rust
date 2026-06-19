use serde::{Deserialize, Serialize};

/// Runtime
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Runtime {
    /// Runtime ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Base Docker image used to build the runtime.
    #[serde(rename = "base", default)]
    pub base: String,
    /// Image name of Docker Hub.
    #[serde(rename = "image", default)]
    pub image: String,
    /// Parent runtime key.
    #[serde(rename = "key", default)]
    pub key: String,
    /// Name of the logo image.
    #[serde(rename = "logo", default)]
    pub logo: String,
    /// Runtime Name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// List of supported architectures.
    #[serde(rename = "supports", default)]
    pub supports: Vec<String>,
    /// Runtime version.
    #[serde(rename = "version", default)]
    pub version: String,
}
