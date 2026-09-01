use serde::{Deserialize, Serialize};

/// One value per PE-102 block that moves data.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BulkJobType {
}
