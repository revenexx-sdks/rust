use serde::{Deserialize, Serialize};

/// No fields — send `{}`. The cut-off is always now, and what counts as
/// expired follows each reservation's own `expires_at` plus the
/// `reservation_ttl_minutes` setting of the market it belongs to.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReservationSweepRequest {
}
