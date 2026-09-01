use serde::{Deserialize, Serialize};

/// No fields — send `{}`. What counts as low follows each row's own
/// `reorder_point` and the market's `reorder_point_default`, exactly as GET
/// /inventories/reorder-alerts computes it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReorderScanRequest {
}
