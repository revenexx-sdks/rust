use serde::{Deserialize, Serialize};

/// A method needs its identity: code + name.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentMethodCreateRequest {
    /// The machine name of the method, unique per tenant and lower case by
    /// convention ('invoice', 'prepayment', 'card', 'paypal'). It is the string
    /// the checkout asks for, the string every payment stores, and therefore the
    /// one value here that cannot be changed freely: renaming it would leave the
    /// ledger naming something that no longer exists, so it is refused with 409
    /// for as long as any payment names it. Required on create.
    #[serde(rename = "code", default)]
    pub code: String,
    /// Allowed ISO 3166-1 alpha-2 country codes, compared upper-cased against the
    /// buyer country. null or an empty list means unrestricted — the invoice
    /// method this app seeds is restricted to DE, which is why an eligibility call
    /// without a country sees it excluded.
    #[serde(rename = "countries", default)]
    pub countries: Vec<String>,
    /// One line explaining the method where it is offered — payment terms, what
    /// happens after the order. Shown to the buyer, so it is the merchant's
    /// wording rather than the app's.
    #[serde(rename = "description", default)]
    pub description: String,
    /// A disabled method is never eligible and never reaches a checkout. This is
    /// the switch an operator wants: deleting a method the ledger still names —
    /// or renaming its `code` — is refused with 409. Defaults to false, so a
    /// half-configured method cannot reach a checkout by accident.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// The surcharge this method costs the buyer, read as an amount or as a
    /// percentage depending on `fee_type`. Never negative — a discount for
    /// paying a certain way is not expressible here. Defaults to 0.
    #[serde(rename = "fee_amount", default)]
    pub fee_amount: f64,
    /// ISO 4217 code a fixed fee is expressed in. The database bounds the length
    /// at three characters and nothing else, so lower case is stored as written.
    /// Defaults to EUR, and lower case is accepted here exactly as the handlers
    /// accept it.
    #[serde(rename = "fee_currency", default)]
    pub fee_currency: String,
    /// How `fee_amount` applies: 'none' (no surcharge), 'fixed' (that many units
    /// of `fee_currency`) or 'percent' (that share of the order amount). Defaults
    /// to 'none'.
    #[serde(rename = "fee_type", default)]
    pub fee_type: String,
    /// Who moves the money. 'self_managed' — invoice, prepayment — means the
    /// merchant fulfils and reconciles it outside any PSP, and such a payment
    /// authorizes the moment it is created. 'psp' means a configured provider
    /// authorizes, captures and refunds it. Defaults to 'self_managed'; 'psp'
    /// needs a 'provider' to transact.
    #[serde(rename = "kind", default)]
    pub kind: String,
    /// Buyer-facing names keyed by language tag — what a storefront shows
    /// instead of the operator-facing `name`. Free jsonb: the database constrains
    /// neither the tags nor the values, so a client reads the tag it wants and
    /// falls back to `en`.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Largest order amount this method may be used for — the usual credit-risk
    /// cap on invoice and prepayment. null means no upper bound.
    #[serde(rename = "max_order_value", default)]
    pub max_order_value: f64,
    /// Free-form merchant data carried on the configuration. This app never reads
    /// it — it is storage for the integrations that do (an ERP key for the
    /// method, a ledger account, a display hint).
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// Smallest order amount this method may be used for — the usual guard
    /// against paying a €5 order by invoice. null means no lower bound.
    #[serde(rename = "min_order_value", default)]
    pub min_order_value: f64,
    /// Operator-facing name, in the language the merchant administers in. What a
    /// buyer sees comes from `labels`. Required on create.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Sort order at checkout, ascending — the merchant's preferred payment
    /// method first. Defaults to 0.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// The PSP code this method transacts through, from GET
    /// /payments/providers/catalog. Only meaningful for kind 'psp'; a PSP method
    /// that names none falls back to the tenant's `default_provider` setting. Must
    /// be a code GET /payments/providers/catalog carries.
    #[serde(rename = "provider", default)]
    pub provider: String,
    /// The provider's own payment-method id ('card', 'paypal', 'sepa_debit') —
    /// what the driver is told to charge. Copied onto every payment created with
    /// this method as `metadata.provider_method`.
    #[serde(rename = "provider_method", default)]
    pub provider_method: String,
}
