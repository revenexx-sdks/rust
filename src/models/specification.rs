use serde::{Deserialize, Serialize};

/// Specification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Specification {
    /// Number of CPUs.
    #[serde(rename = "cpus", default)]
    pub cpus: f64,
    /// Is size enabled.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// Memory size in MB.
    #[serde(rename = "memory", default)]
    pub memory: i64,
    /// Size slug.
    #[serde(rename = "slug", default)]
    pub slug: String,
}
