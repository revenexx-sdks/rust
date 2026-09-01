use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Payment {
    /// What the provider is asked to authorize, in `currency`. 0 is legal (a free
    /// order) and negative is refused by the handler and by the CHECK behind it.
    /// `fee_amount` is recorded beside this and is NOT added to it — a checkout
    /// that charges its payment surcharge sends a total that already includes it.
    #[serde(rename = "amount", default)]
    pub amount: f64,
    /// When the money was reserved — or, for invoice and prepayment, when it
    /// became owed. The clock the capture window and the dunning stages are
    /// measured from.
    #[serde(rename = "authorized_at", default)]
    pub authorized_at: String,
    /// When the money was actually taken. The refund window is measured from here.
    #[serde(rename = "captured_at", default)]
    pub captured_at: String,
    /// The cart this payment pays for. Not a foreign key: the payment is a record
    /// of what happened and outlives the cart. Indexed, so it is the cheap way to
    /// find the payment behind a checkout.
    #[serde(rename = "cart_id", default)]
    pub cart_id: String,
    /// The paying customer contact. Not a foreign key — a payment must survive a
    /// contact being merged or erased. Indexed.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// When the payment was created. The dunning clock for invoice and prepayment
    /// runs from here.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// ISO 4217 code the amount and the fee are in. The database bounds the length
    /// at three characters and nothing else, so lower case is stored as written.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// When the NEXT dunning stage falls due — the moment a reminder becomes
    /// due, then the moment it becomes overdue. null once nothing further is
    /// pending, which includes an already overdue payment and every paid,
    /// cancelled or refunded one.
    #[serde(rename = "dunning_due_at", default)]
    pub dunning_due_at: String,
    /// How overdue an unpaid self-managed payment is: 'none', 'reminder' or
    /// 'overdue'. Written by the daily dunning scan from the merchant's two
    /// thresholds, and reset the moment the money arrives or the claim is dropped.
    /// It classifies and never sends: what a reminder looks like is the merchant's
    /// own workflow.
    #[serde(rename = "dunning_stage", default)]
    pub dunning_stage: String,
    /// The class of failure, out of a fixed taxonomy — the value to branch on.
    /// null unless the payment failed. The five classes say what a caller can DO:
    /// 'provider_unavailable', 'provider_unreachable', 'provider_not_configured',
    /// 'provider_declined', 'provider_error' — a provider that is unreachable or
    /// unavailable is worth a retry, a declined payment needs a different method
    /// from the buyer, and a provider that is not configured needs an operator.
    #[serde(rename = "error_code", default)]
    pub error_code: String,
    /// One operator-facing sentence, fixed per `error_code`. Never the provider's
    /// or the runtime's own wording: that is unbounded internal text and it stays
    /// in the app log.
    #[serde(rename = "error_message", default)]
    pub error_message: String,
    /// When the payment failed. `error_code` says which class of failure.
    #[serde(rename = "failed_at", default)]
    pub failed_at: String,
    /// The method surcharge as it was computed at creation, in `currency`. Kept so
    /// the fee that was quoted stays readable after the method's fee configuration
    /// changes.
    #[serde(rename = "fee_amount", default)]
    pub fee_amount: f64,
    /// Id of the payment. Every lifecycle route addresses it, and it is what the
    /// drivers send the provider as their merchant transaction reference.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The caller's own key for this creation attempt. Sending it again answers
    /// the SAME payment with 200 instead of creating a second one — which is
    /// what makes a retried checkout safe. Unique per tenant, so a filter on it
    /// answers at most one row.
    #[serde(rename = "idempotency_key", default)]
    pub idempotency_key: String,
    /// Copied from the method at creation. 'self_managed' payments move through
    /// the lifecycle without a PSP; 'psp' payments are driven by `provider`.
    #[serde(rename = "kind", default)]
    pub kind: String,
    /// Whatever the creating call sent, plus the keys this app writes onto it. The
    /// app's own: `provider_method` (the method's provider-side id, copied at
    /// creation), `return_url` (where the PSP sends the buyer back),
    /// `cancel_reason` / `refund_reason` (the operator's words from the cancel and
    /// refund routes, also handed to the provider) and `provider_fallback_from`
    /// (the provider that was WANTED, written when the tenant's fallback_provider
    /// stood in — the only record of why the money went through a different
    /// acquirer). Free jsonb; a caller's own keys are kept untouched beside these.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// The `code` of the payment method this payment was made with, copied at
    /// creation. Deliberately a code and not a foreign key: the ledger records
    /// what happened and has to outlive the configuration it happened under.
    #[serde(rename = "method_code", default)]
    pub method_code: String,
    /// What the storefront must do before this payment can go any further, or null
    /// when there is nothing to do. It is set exactly when `status` is
    /// `requires_action`, and every transition clears it. One shape exists today:
    /// `{ "type": "redirect", "url": … }` — send the buyer to `url` (that is
    /// also where a 3-D Secure challenge is presented, because the connector hands
    /// it back as a redirect), and when they come back call POST
    /// /payments/{id}/confirm. `type` is what to branch on; a client that does not
    /// recognise it must not guess.
    #[serde(rename = "next_action", default)]
    pub next_action: serde_json::Value,
    /// The external order reference the checkout wrote onto the payment. It is
    /// what POST /payments/orders/{order_ref}/capture resolves and the fallback
    /// key a PSP webhook is matched on when it carries no transaction id — so an
    /// integration that leaves it null gives up both. Free text with no
    /// uniqueness: several payments may share one reference.
    #[serde(rename = "order_ref", default)]
    pub order_ref: String,
    /// The PSP the money really went through — resolved at creation and
    /// rewritten if the tenant's fallback provider stood in, in which case
    /// `metadata.provider_fallback_from` records what was meant. null for
    /// self-managed payments.
    #[serde(rename = "provider", default)]
    pub provider: String,
    /// The provider's own transaction id, as it answered — the value to quote in
    /// a PSP support case, and the primary key a webhook is matched on. Shaped by
    /// the provider, so nothing here constrains it; null until a provider has
    /// answered, and always null for self-managed payments.
    #[serde(rename = "psp_payment_id", default)]
    pub psp_payment_id: String,
    /// When the payment was refunded in full — this app has no partial refund to
    /// record.
    #[serde(rename = "refunded_at", default)]
    pub refunded_at: String,
    /// Where the payment stands. 'created' → 'requires_action' → 'authorized'
    /// → 'captured' → 'refunded', with 'failed' and 'cancelled' ending it. GET
    /// /payments/vocabularies/statuses serves the same set with labels, badge
    /// tones and which of them are final.
    #[serde(rename = "status", default)]
    pub status: String,
    /// The tenant the row belongs to — the same slug the request carried in
    /// `X-Revenexx-Tenant`. Added by the platform rather than by this app, and
    /// echoed so a caller that fans several tenants into one store can tell the
    /// rows apart.
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    /// When the row last moved. For a PSP payment still waiting on a callback this
    /// is what the webhook-staleness check measures against, so an old payment
    /// that changed a minute ago counts as progressing.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
