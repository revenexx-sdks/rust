use serde::{Deserialize, Serialize};

/// What the repair changed. `kept` + `added` + `seeded` is what the market now
/// holds, and separating them is the point: it shows that nothing the merchant
/// had already decided was touched.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketBackfillResult {
    /// Child rows copied in from the source, per collection — only codes this
    /// market did not already carry. Zero everywhere on a second run: the call is
    /// idempotent.
    #[serde(rename = "added", default)]
    pub added: crate::models::MarketBackfillAdded,
    /// What this market already held BEFORE the repair, per collection — the
    /// rows that were left exactly as the merchant left them.
    #[serde(rename = "kept", default)]
    pub kept: crate::models::MarketBackfillKept,
    /// A distinct business context within a tenant — a country, a region, or a
    /// storefront segment such as B2C vs B2B — with its own base currency,
    /// locales, traded currencies and tax classes. A market is also the platform's
    /// `market` SCOPE dimension: every other commerce app slices its data by one,
    /// keyed on this row's `code`. A market is never just this row: it needs at
    /// least one locale, one currency and one tax class before it can serve, which
    /// is what /readiness measures and what /clone and /backfill build.
    #[serde(rename = "market", default)]
    pub market: crate::models::Market,
    /// Can this market actually trade? `ready` is false only when a BLOCKING check
    /// failed — no currency to quote in, no tax class to tax with. Warnings are
    /// degraded-but-serviceable.
    #[serde(rename = "readiness", default)]
    pub readiness: crate::models::MarketReadiness,
    /// Rows this call added that were copied from nowhere, because the new market
    /// would otherwise have been left unable to trade: the tenant
    /// `fallback_locale` when neither market had a locale, and the base currency
    /// when it is not in the copied set. Zero on both is the normal, healthy
    /// answer — it means nothing had to be invented.
    #[serde(rename = "seeded", default)]
    pub seeded: crate::models::MarketBackfillSeeded,
    /// The market that was read from, resolved — so a caller who passed a code
    /// back gets the uuid, and one who passed a uuid gets the code the rest of the
    /// platform stores.
    #[serde(rename = "source", default)]
    pub source: crate::models::MarketRef,
}
