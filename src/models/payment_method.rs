use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentMethod {
    /// The machine name of the method, unique per tenant and lower case by
    /// convention ('invoice', 'prepayment', 'card', 'paypal'). It is the string
    /// the checkout asks for, the string every payment stores, and therefore the
    /// one value here that cannot be changed freely: renaming it would leave the
    /// ledger naming something that no longer exists, so it is refused with 409
    /// for as long as any payment names it.
    #[serde(rename = "code", default)]
    pub code: String,
    /// Allowed ISO 3166-1 alpha-2 country codes, compared upper-cased against the
    /// buyer country. null or an empty list means unrestricted — the invoice
    /// method this app seeds is restricted to DE, which is why an eligibility call
    /// without a country sees it excluded.
    #[serde(rename = "countries", default)]
    pub countries: Vec<String>,
    /// When this configuration was created.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// One line explaining the method where it is offered — payment terms, what
    /// happens after the order. Shown to the buyer, so it is the merchant's
    /// wording rather than the app's.
    #[serde(rename = "description", default)]
    pub description: String,
    /// A disabled method is never eligible and never reaches a checkout. This is
    /// the switch an operator wants: deleting a method the ledger still names —
    /// or renaming its `code` — is refused with 409.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// The surcharge this method costs the buyer, read as an amount or as a
    /// percentage depending on `fee_type`. Never negative — a discount for
    /// paying a certain way is not expressible here.
    #[serde(rename = "fee_amount", default)]
    pub fee_amount: f64,
    /// ISO 4217 code a fixed fee is expressed in. The database bounds the length
    /// at three characters and nothing else, so lower case is stored as written.
    #[serde(rename = "fee_currency", default)]
    pub fee_currency: String,
    /// How `fee_amount` applies: 'none' (no surcharge), 'fixed' (that many units
    /// of `fee_currency`) or 'percent' (that share of the order amount).
    #[serde(rename = "fee_type", default)]
    pub fee_type: String,
    /// Id of the configuration row. A payment names its method by `code`, never by
    /// this — so an id is only ever used to address the configuration itself.
    #[serde(rename = "id", default)]
    pub id: String,
    /// Who moves the money. 'self_managed' — invoice, prepayment — means the
    /// merchant fulfils and reconciles it outside any PSP, and such a payment
    /// authorizes the moment it is created. 'psp' means a configured provider
    /// authorizes, captures and refunds it.
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
    /// buyer sees comes from `labels`.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Sort order at checkout, ascending — the merchant's preferred payment
    /// method first.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// The PSP code this method transacts through, from GET
    /// /payments/providers/catalog. Only meaningful for kind 'psp'; a PSP method
    /// that names none falls back to the tenant's `default_provider` setting.
    #[serde(rename = "provider", default)]
    pub provider: String,
    /// The provider's own payment-method id ('card', 'paypal', 'sepa_debit') —
    /// what the driver is told to charge. Copied onto every payment created with
    /// this method as `metadata.provider_method`.
    #[serde(rename = "provider_method", default)]
    pub provider_method: String,
    /// The tenant the row belongs to — the same slug the request carried in
    /// `X-Revenexx-Tenant`. Added by the platform rather than by this app, and
    /// echoed so a caller that fans several tenants into one store can tell the
    /// rows apart.
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    /// When it was last changed. The eligibility answer is computed live, so this
    /// is the age of the configuration and not of any cached result.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
