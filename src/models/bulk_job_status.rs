use serde::{Deserialize, Serialize};

/// Lifecycle of a `baseline.bulk_jobs` row:
/// `pending → running → completed`, or `partial` (finished with
/// `counts.rejected > 0`), `failed`, or `canceled`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BulkJobStatus {
}
