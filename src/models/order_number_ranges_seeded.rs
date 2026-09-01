use serde::{Deserialize, Serialize};

/// Which of the three standard codes this call had to create and which were
/// already there.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrderNumberRangesSeeded {
    /// The codes that were created just now, with the standard format
    /// ORD-/DEL-/RET- and padding 6. Empty on every call after the first.
    #[serde(rename = "created", default)]
    pub created: Vec<String>,
    /// The codes that were already there and were left EXACTLY as they are — a
    /// merchant who changed the prefix or the counter keeps their change. That is
    /// what makes this call safe to run again.
    #[serde(rename = "existing", default)]
    pub existing: Vec<String>,
}
