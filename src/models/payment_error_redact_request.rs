use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentErrorRedactRequest {
    /// Write the reclassified values. Defaults to false, which reports what WOULD
    /// change and touches nothing.
    #[serde(rename = "apply", default)]
    pub apply: bool,
    /// How many payments to scan, oldest first. Defaults to 500, capped at 5000
    /// — a tenant with more pre-taxonomy rows needs several runs, and re-running
    /// is free.
    #[serde(rename = "limit", default)]
    pub limit: i64,
}
