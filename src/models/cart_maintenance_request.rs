use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartMaintenanceRequest {
    /// Report what the sweep WOULD do and write nothing. Worth doing before a
    /// first retention run: cart_ttl_days deletes carts and their lines.
    #[serde(rename = "dry_run", default)]
    pub dry_run: bool,
}
