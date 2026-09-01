use serde::{Deserialize, Serialize};

/// The second sweep, and the only destructive thing this app does: carts past
/// their retention window are deleted, their lines with them. An ordered cart
/// is never touched at any setting — it is the source record of a sale.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartPurgeSweep {
    /// More carts were available to examine than one pass examines; the rest go
    /// next tick, oldest first.
    #[serde(rename = "capped", default)]
    pub capped: bool,
    /// The carts this sweep touched, so a merchant can look at them before or
    /// after.
    #[serde(rename = "cart_ids", default)]
    pub cart_ids: Vec<String>,
    /// The tenant baseline's window for CUSTOMER carts, in days. 0 is 'never
    /// delete' — the default, and also where an unparsable value lands, so no
    /// settings outage can start a purge.
    #[serde(rename = "cart_ttl_days", default)]
    pub cart_ttl_days: f64,
    /// The baseline cutoff, for carts belonging to no market. Null when the
    /// baseline keeps everything.
    #[serde(rename = "cutoff", default)]
    pub cutoff: String,
    /// Carts actually deleted. 0 on a dry run — see `found`.
    #[serde(rename = "deleted", default)]
    pub deleted: i64,
    /// Retention was in force for at least one cart this pass looked at — the
    /// baseline, or some market that sets a window while the baseline leaves it
    /// off. False means nothing could have been deleted.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// Carts past their retention window. On a dry run this is what the wet run
    /// would remove.
    #[serde(rename = "found", default)]
    pub found: i64,
    /// The same for GUEST carts — a cart with a session key and no contact
    /// behind it. Kept separate because the two are worth different amounts: a
    /// named B2B cart may be a quote somebody is still thinking about.
    #[serde(rename = "guest_cart_ttl_days", default)]
    pub guest_cart_ttl_days: f64,
    /// Lines actually deleted with them. 0 on a dry run.
    #[serde(rename = "items_deleted", default)]
    pub items_deleted: i64,
    /// The market codes this pass came across. Each cart was held against ITS
    /// market's window, not the baseline's.
    #[serde(rename = "markets", default)]
    pub markets: Vec<String>,
    /// Lines the wet run would remove. Always present, on a wet run too, so a
    /// client never has to tell "nothing to delete" apart from "this build did not
    /// report it".
    #[serde(rename = "would_delete_items", default)]
    pub would_delete_items: i64,
}
