use serde::{Deserialize, Serialize};

/// One question asked of the market, its verdict, and how much the answer
/// costs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketReadinessCheck {
    /// One sentence naming what was found and, for a warning, what covers for it.
    #[serde(rename = "detail", default)]
    pub detail: String,
    /// Which question. 'locales' — is there a language to render in?
    /// 'currencies' — is the base currency registered and marked default?
    /// 'tax_classes' — is there a rate to tax with? 'tax_basis' —
    /// informational, restating whether stored prices are gross or net.
    #[serde(rename = "id", default)]
    pub id: String,
    /// Whether this check passed. A false with severity `info` cannot occur —
    /// the informational check always passes.
    #[serde(rename = "ok", default)]
    pub ok: bool,
    /// What a failure costs. 'blocking' — the market cannot trade. 'warning' —
    /// degraded but serviceable, and `detail` names what covers for it. 'info' —
    /// a fact worth reporting that is never a failure. The severity is not fixed
    /// per check: no locales is blocking without a tenant fallback_locale and a
    /// warning with one.
    #[serde(rename = "severity", default)]
    pub severity: String,
}
