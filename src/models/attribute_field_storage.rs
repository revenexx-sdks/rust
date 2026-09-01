use serde::{Deserialize, Serialize};

/// Where the value lives. Absent on an app whose custom fields are plain
/// columns — then the field name IS the column.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributeFieldStorage {
    /// Which scope bucket this attribute writes to, implied by
    /// localizable/scopable.
    #[serde(rename = "bucket", default)]
    pub bucket: String,
    /// The jsonb column holding the values (`attribute_values`).
    #[serde(rename = "column", default)]
    pub column: String,
    /// The exact key path for the requested context, or null when the request
    /// named no locale/channel and the bucket needs one. Null means: read-only
    /// until a context is chosen.
    #[serde(rename = "path", default)]
    pub path: Vec<String>,
}
