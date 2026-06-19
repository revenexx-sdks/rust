use serde::{Deserialize, Serialize};

/// Files List
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileList {
    /// List of files.
    #[serde(rename = "files", default)]
    pub files: Vec<crate::models::File>,
    /// Total number of files that matched your query.
    #[serde(rename = "total", default)]
    pub total: i64,
}
