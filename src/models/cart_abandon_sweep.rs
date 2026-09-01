use serde::{Deserialize, Serialize};

/// The first sweep: active carts nobody has touched since their market's
/// window become abandoned. Nothing else in the platform ever stamps
/// abandoned_at, so without this the abandonment funnel is empty by
/// construction rather than empty because nobody abandons carts.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartAbandonSweep {
    /// Carts actually marked. 0 on a dry run — see `found`.
    #[serde(rename = "abandoned", default)]
    pub abandoned: i64,
    /// The abandon_after_minutes of the TENANT baseline — what a cart in no
    /// market ran on. 0 disables the sweep. Carts in a market were each held
    /// against their own market's window, which may differ from this.
    #[serde(rename = "after_minutes", default)]
    pub after_minutes: f64,
    /// This pass looked at as many carts as one pass looks at, so there may be
    /// more behind them. The rest go on the next tick, oldest first — a backlog
    /// is visible here rather than merely slow.
    #[serde(rename = "capped", default)]
    pub capped: bool,
    /// The carts this sweep touched, so a merchant can look at them before or
    /// after.
    #[serde(rename = "cart_ids", default)]
    pub cart_ids: Vec<String>,
    /// Carts untouched since this instant were swept — the BASELINE cutoff. A
    /// run no longer has one cutoff, because each cart was held against its own
    /// market's clock; this is the one unassigned carts ran on.
    #[serde(rename = "cutoff", default)]
    pub cutoff: String,
    /// At least one window in force (the baseline, or some market's). False means
    /// every applicable window was 0 and nothing was even considered.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// Carts past their window. On a dry run this is the whole answer —
    /// `abandoned` stays 0.
    #[serde(rename = "found", default)]
    pub found: i64,
    /// The market codes this pass came across, so an operator can see whose
    /// windows were actually in play. Empty when no examined cart belongs to a
    /// market.
    #[serde(rename = "markets", default)]
    pub markets: Vec<String>,
}
