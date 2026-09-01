use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartMaintenanceResult {
    /// The first sweep: active carts nobody has touched since their market's
    /// window become abandoned. Nothing else in the platform ever stamps
    /// abandoned_at, so without this the abandonment funnel is empty by
    /// construction rather than empty because nobody abandons carts.
    #[serde(rename = "abandon", default)]
    pub abandon: crate::models::CartAbandonSweep,
    /// This pass wrote nothing. The counts and cart ids are the same ones the wet
    /// run would produce.
    #[serde(rename = "dry_run", default)]
    pub dry_run: bool,
    /// The second sweep, and the only destructive thing this app does: carts past
    /// their retention window are deleted, their lines with them. An ordered cart
    /// is never touched at any setting — it is the source record of a sale.
    #[serde(rename = "purge", default)]
    pub purge: crate::models::CartPurgeSweep,
    /// The instant this pass measured every window against. One clock for both
    /// sweeps, so a cart cannot be judged idle by one and fresh by the other.
    #[serde(rename = "swept_at", default)]
    pub swept_at: String,
}
