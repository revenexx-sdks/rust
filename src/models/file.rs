use serde::{Deserialize, Serialize};

/// File
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct File {
    /// File creation date in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// File ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// File permissions. Each entry is a permission string: an action wrapping a
    /// role, e.g. `read("any")`, `update("user:abc")`, `delete("team:abc/owner")`.
    /// Actions are `read`, `create`, `update`, `delete` and the aggregate `write`
    /// (= create + update + delete); the role inside the quotes takes the form
    /// described under “Role strings” in this document's introduction.
    #[serde(rename = "$permissions", default)]
    pub permissions: Vec<String>,
    /// File update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Bucket ID.
    #[serde(rename = "bucketId", default)]
    pub bucket_id: String,
    /// Total number of chunks available
    #[serde(rename = "chunksTotal", default)]
    pub chunks_total: i64,
    /// Total number of chunks uploaded
    #[serde(rename = "chunksUploaded", default)]
    pub chunks_uploaded: i64,
    /// Compression algorithm used for the file. Will be one of none,
    /// [gzip](https://en.wikipedia.org/wiki/Gzip), or
    /// [zstd](https://en.wikipedia.org/wiki/Zstd).
    #[serde(rename = "compression", default)]
    pub compression: String,
    /// Whether file contents are encrypted at rest.
    #[serde(rename = "encryption", default)]
    pub encryption: bool,
    /// File mime type.
    #[serde(rename = "mimeType", default)]
    pub mime_type: String,
    /// File name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// File MD5 signature.
    #[serde(rename = "signature", default)]
    pub signature: String,
    /// File original size in bytes.
    #[serde(rename = "sizeOriginal", default)]
    pub size_original: i64,
}
