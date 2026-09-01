use serde::{Deserialize, Serialize};

/// Creates AND authorizes: self-managed methods authorize immediately, PSP
/// methods may answer next_action (redirect). Eligibility is re-checked
/// server-side.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentCreateRequest {
    /// What the provider is asked to authorize, in `currency`. 0 is legal (a free
    /// order) and negative is refused by the handler and by the CHECK behind it.
    /// `fee_amount` is recorded beside this and is NOT added to it — a checkout
    /// that charges its payment surcharge sends a total that already includes it.
    #[serde(rename = "amount", default)]
    pub amount: f64,
    /// The cart this payment pays for. Not a foreign key: the payment is a record
    /// of what happened and outlives the cart. Indexed, so it is the cheap way to
    /// find the payment behind a checkout.
    #[serde(rename = "cart_id", default)]
    pub cart_id: String,
    /// The paying customer contact. Not a foreign key — a payment must survive a
    /// contact being merged or erased. Indexed.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// The buyer's ISO 3166-1 alpha-2 country code, for the eligibility check. A
    /// method restricted to countries is refused with 422 without it.
    #[serde(rename = "country", default)]
    pub country: String,
    /// ISO 4217 code the amount and the fee are in. The database bounds the length
    /// at three characters and nothing else, so lower case is stored as written.
    /// Defaults to EUR.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// The caller's own key for this creation attempt. Sending it again answers
    /// the SAME payment with 200 instead of creating a second one — which is
    /// what makes a retried checkout safe. Unique per tenant, so a filter on it
    /// answers at most one row. The replay answers 200, not 201.
    #[serde(rename = "idempotency_key", default)]
    pub idempotency_key: String,
    /// Free-form data to keep on the payment. Merged with the keys this app writes
    /// itself (`provider_method`, `return_url`, later the cancel/refund reasons),
    /// which win on a collision.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// The `code` of the payment method this payment was made with, copied at
    /// creation. Deliberately a code and not a foreign key: the ledger records
    /// what happened and has to outlive the configuration it happened under. It
    /// must name a method this tenant has configured; eligibility for the buyer
    /// context below is re-checked here, whatever the checkout showed.
    #[serde(rename = "method_code", default)]
    pub method_code: String,
    /// The external order reference the checkout wrote onto the payment. It is
    /// what POST /payments/orders/{order_ref}/capture resolves and the fallback
    /// key a PSP webhook is matched on when it carries no transaction id — so an
    /// integration that leaves it null gives up both. Free text with no
    /// uniqueness: several payments may share one reference.
    #[serde(rename = "order_ref", default)]
    pub order_ref: String,
    /// Where the PSP sends the buyer back after a redirect or a 3-D Secure
    /// challenge. Kept in `metadata.return_url` and handed to the driver — a PSP
    /// method that needs a redirect and has none leaves the buyer stranded at the
    /// provider.
    #[serde(rename = "return_url", default)]
    pub return_url: String,
}
