use serde::{Deserialize, Serialize};

/// Can this market actually trade? `ready` is false only when a BLOCKING check
/// failed — no currency to quote in, no tax class to tax with. Warnings are
/// degraded-but-serviceable. The market it is about, what it is made of, and
/// the verdict — the readiness block is inlined here rather than nested, so
/// this is MarketReadiness plus two keys.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketReadinessReport {
    /// Ids of the checks that failed BLOCKING — the market cannot do the job at
    /// all until each is fixed. Empty exactly when `ready` is true.
    #[serde(rename = "blocking", default)]
    pub blocking: Vec<String>,
    /// Every check that ran, passed or failed, in a fixed order: locales,
    /// currencies, tax_classes, tax_basis. `blocking` and `warnings` are the
    /// failures from this list by id; this is where the reason lives.
    #[serde(rename = "checks", default)]
    pub checks: Vec<crate::models::MarketReadinessCheck>,
    /// How much of a market this market actually is. All three at zero is a market
    /// that is a row and nothing else — the state two of the three live markets
    /// on the platform were left in, and the reason /clone and /backfill exist.
    #[serde(rename = "counts", default)]
    pub counts: crate::models::MarketReadinessCounts,
    /// The market the verdict is about, identified rather than returned in full
    /// — the five columns a reader needs to know which market answered. Read GET
    /// /markets/{id} for the rest.
    #[serde(rename = "market", default)]
    pub market: crate::models::MarketReadinessSubject,
    /// `blocking` is empty. Deliberately not "every check passed": a market with
    /// one locale and no default flag on it is serviceable, and a verdict that
    /// cried wolf about that would be ignored on the day it mattered.
    #[serde(rename = "ready", default)]
    pub ready: bool,
    /// true when the market's status is 'active'. An active market that is not
    /// ready is live and broken — that combination is the one worth an alert.
    #[serde(rename = "serving", default)]
    pub serving: bool,
    /// Ids of the checks that failed as WARNINGS — degraded but serviceable,
    /// because something else covers for them. A missing locale is only a warning
    /// while the tenant declares a fallback_locale.
    #[serde(rename = "warnings", default)]
    pub warnings: Vec<String>,
}
