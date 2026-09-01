use serde::{Deserialize, Serialize};

/// Bucket
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Bucket {
    /// Bucket creation time in ISO 8601 format.
    #[serde(rename = "$createdAt", default)]
    pub created_at: String,
    /// Bucket ID.
    #[serde(rename = "$id", default)]
    pub id: String,
    /// Bucket permissions. Each entry is a permission string: an action wrapping a
    /// role, e.g. `read("any")`, `update("user:abc")`, `delete("team:abc/owner")`.
    /// Actions are `read`, `create`, `update`, `delete` and the aggregate `write`
    /// (= create + update + delete); the role inside the quotes takes the form
    /// described under “Role strings” in this document's introduction.
    #[serde(rename = "$permissions", default)]
    pub permissions: Vec<String>,
    /// Bucket update date in ISO 8601 format.
    #[serde(rename = "$updatedAt", default)]
    pub updated_at: String,
    /// Allowed file extensions.
    #[serde(rename = "allowedFileExtensions", default)]
    pub allowed_file_extensions: Vec<String>,
    /// Virus scanning is enabled.
    #[serde(rename = "antivirus", default)]
    pub antivirus: bool,
    /// Compression algorithm chosen for compression. Will be one of none,
    /// [gzip](https://en.wikipedia.org/wiki/Gzip), or
    /// [zstd](https://en.wikipedia.org/wiki/Zstd).
    #[serde(rename = "compression", default)]
    pub compression: String,
    /// Bucket enabled.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// Bucket is encrypted.
    #[serde(rename = "encryption", default)]
    pub encryption: bool,
    /// Whether file-level security is enabled. When it is, each record's own
    /// `$permissions` are enforced on top of the container's.
    #[serde(rename = "fileSecurity", default)]
    pub file_security: bool,
    /// Maximum file size supported.
    #[serde(rename = "maximumFileSize", default)]
    pub maximum_file_size: i64,
    /// Bucket name.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Total size of this bucket in bytes.
    #[serde(rename = "totalSize", default)]
    pub total_size: i64,
    /// Image transformations are enabled.
    #[serde(rename = "transformations", default)]
    pub transformations: bool,
}
