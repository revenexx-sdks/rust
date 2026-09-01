use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormActionMapping {
    /// The key in the submission `data` — i.e. the `name` of a definition node.
    #[serde(rename = "source", default)]
    pub source: String,
    /// The column of the target entity it is written to.
    #[serde(rename = "target", default)]
    pub target: String,
}
