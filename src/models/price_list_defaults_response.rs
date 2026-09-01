use serde::{Deserialize, Serialize};

/// What seeding found and what it had to write. Idempotent twice over: by
/// code, and by the existence of ANY default list — so changing
/// default_price_list_code later never produces a second default.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceListDefaultsResponse {
    /// Codes of the lists this call created — empty on a tenant that was already
    /// seeded.
    #[serde(rename = "created", default)]
    pub created: Vec<String>,
    /// Codes of the lists that were already there, so nothing was written for
    /// them.
    #[serde(rename = "existing", default)]
    pub existing: Vec<String>,
}
