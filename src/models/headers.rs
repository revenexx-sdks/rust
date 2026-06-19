use serde::{Deserialize, Serialize};

/// Headers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Headers {
    /// Header name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Header value.
    #[serde(rename = "value", default)]
    pub value: String,
}
