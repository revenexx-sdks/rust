use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// PaymentsLedger service
pub struct PaymentsLedger {
    client: Client,
}

impl PaymentsLedger {
    pub fn new(client: Client) -> Self {
        PaymentsLedger { client }
    }
    /// The ledger, paged and filtered — the Payments screen, the reconciliation
    /// query and the way an order or a cart finds out what has been paid against
    /// it. Every column of the entity is an exact-match filter, which is what
    /// makes it useful: `?cart_id=` and `?contact_id=` are indexed,
    /// `?status=authorized&kind=self_managed` is the awaiting-payment queue the
    /// dunning scan classifies, and `?order_ref=` is the only way to resolve a
    /// payment by its external reference. Rows come back in the database's own
    /// order, so a newest-first list needs `?order=created_at.desc`.
    /// `error_message` is answered from the failure taxonomy rather than echoed
    /// out of the column, so what a driver or a PSP actually wrote is never
    /// serialized here.
    pub async fn payments_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, cart_id: Option<String>, contact_id: Option<String>, status: Option<String>, order_ref: Option<String>, method_code: Option<String>, kind: Option<String>, provider: Option<String>, dunning_stage: Option<String>, idempotency_key: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/payments".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &offset {
            api_params.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order {
            api_params.insert("order".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &cart_id {
            api_params.insert("cart_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &contact_id {
            api_params.insert("contact_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order_ref {
            api_params.insert("order_ref".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &method_code {
            api_params.insert("method_code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &kind {
            api_params.insert("kind".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &provider {
            api_params.insert("provider".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &dunning_stage {
            api_params.insert("dunning_stage".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &idempotency_key {
            api_params.insert("idempotency_key".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// The checkout's write: it opens the ledger row and takes it as far as the
    /// named method allows, in one call. A create cannot omit `method_code` and
    /// `amount`; every other column is optional or defaulted by the database.
    /// Nothing else about the money is the caller's to choose: `kind`, `provider`
    /// and `fee_amount` are read off the method that `method_code` names, so a
    /// caller can neither pick an acquirer nor discount its own fee. `amount: 0`
    /// is legal (free orders); negative is 400. Eligibility is enforced HERE and
    /// not only in the checkout UI — the same country and order-value rules POST
    /// /payments/methods/eligible applies answer 422 if the method does not apply
    /// to this buyer. What comes back depends on the method: a self-managed one
    /// (invoice, prepayment) is `authorized` at once with the dunning clock
    /// already started, and a PSP one is `captured` or `authorized`, or
    /// `requires_action` with `next_action` — the instruction the storefront
    /// must carry out, typically a redirect, set at that status and at no other.
    /// Send an `idempotency_key` and a repeat of the same call answers 200 with
    /// the payment that key already named, unchanged and not re-authorized. What
    /// is never stored: the `instrument`, `token` or `card` is handed to the
    /// driver in-process and no token or PAN is written to the row.
    pub async fn payments_create(&self, amount: f64, method_code: String, cart_id: Option<String>, contact_id: Option<String>, country: Option<String>, currency: Option<String>, idempotency_key: Option<String>, metadata: Option<serde_json::Value>, order_ref: Option<String>, return_url: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/payments".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("amount".to_string(), serde_json::to_value(&amount)?);
        api_params.insert("method_code".to_string(), serde_json::to_value(&method_code)?);
        if let Some(value) = &cart_id {
            api_params.insert("cart_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &contact_id {
            api_params.insert("contact_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &country {
            api_params.insert("country".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &currency {
            api_params.insert("currency".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &idempotency_key {
            api_params.insert("idempotency_key".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &metadata {
            api_params.insert("metadata".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order_ref {
            api_params.insert("order_ref".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &return_url {
            api_params.insert("return_url".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Classifies every unpaid self-managed payment (invoice, prepayment) as on
    /// time / reminder due / overdue from payment_reminder_after_days and
    /// overdue_after_days, writes the stage and the next due date, and reports PSP
    /// payments still waiting on a callback longer than
    /// webhook_stale_after_minutes. Pure function of each payment's age, so it is
    /// idempotent — it also runs daily as the 'dunning-scan' schedule. It
    /// classifies and does not send: a stage change emits payment.updated, and
    /// what a reminder looks like is the merchant's workflow.
    pub async fn payments_dunning_scan(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/payments/dunning/scan".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// Rows written before the failure taxonomy still store the
    /// provider's/runtime's raw text in error_message. API responses never repeat
    /// it (the read path projects), but the column is also read directly through
    /// Baseline, so it needs rewriting once per tenant. Dry-run by default —
    /// reports what it would touch and changes nothing until apply:true.
    /// Idempotent: rows already carrying a taxonomy message are skipped.
    pub async fn payments_errors_redact(&self, apply: Option<bool>, limit: Option<i64>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/payments/errors/redact".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &apply {
            api_params.insert("apply".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// This is the hook the tenant's `auto_capture_policy: 'on_ship'` was written
    /// for: fulfilment knows the order it shipped and not the payment ids behind
    /// it, so the shipment calls this one route with the reference it already
    /// holds and the money for that order is collected in a single request.
    /// Resolves payments by their order_ref (the same key the PSP webhooks fall
    /// back to), captures every authorized one and reports the rest instead of
    /// failing — an order whose payment was already captured is a successful
    /// no-op, and a provider that refuses one payment lands in `skipped` rather
    /// than failing the call. Note that payments.order_ref is nullable with no
    /// foreign key: this route is exactly as good as the reference the checkout
    /// writes onto the payment.
    pub async fn payments_orders_capture(&self, order_ref: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/payments/orders/{order_ref}/capture".replace("{order_ref}", &order_ref.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("order_ref".to_string(), serde_json::to_value(&order_ref)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The enums this app owns, four of them: statuses, method kinds, fee types
    /// and dunning stages. This is the index and carries a name and a title per
    /// set and nothing more — the values themselves, with their labels and badge
    /// tones, are one call further down at GET /payments/vocabularies/{name}, so a
    /// client that only needs to know which sets exist does not pay for all of
    /// them. Values come out of the CHECK constraints, so what is served is what
    /// the database enforces — a client renders a status this app adds without a
    /// release of its own.
    pub async fn payments_vocabularies_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/payments/vocabularies".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// One set in full: every value it permits, the label to show for each and the
    /// badge tone to render it in, which is what a client needs to draw a status
    /// chip without hard-coding this app's enums. The value set is parsed out of
    /// the CHECK constraint in schema.json, so what is served IS what the database
    /// enforces. Labels are curated on top and can only add words and colour — a
    /// permitted value nobody labelled still appears, titled from its own key,
    /// which is why `title` and `description` are a locale map on a labelled value
    /// and a plain string on an unlabelled one.
    pub async fn payments_vocabularies_get(&self, name: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/payments/vocabularies/{name}".replace("{name}", &name.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The sink a PSP callback ends up in, and an inbound ingress endpoint in the
    /// sense of ADR-0066: the provider never posts here directly, it posts to
    /// webhooks.revenexx.com, which verifies and captures the delivery and
    /// dispatches its envelope to this route through the gateway. That indirection
    /// is also what makes this the one override point for PSP callback handling
    /// — everything a callback does to the ledger happens here and nowhere else,
    /// so a deployment that needs a provider's callbacks normalized differently
    /// replaces this operation instead of touching the lifecycle routes. Consumes
    /// the dispatch envelope from webhooks.revenexx.com: normalizes the provider
    /// callback (stripe payment intents + a generic shape), resolves the payment
    /// by psp_payment_id or order_ref and moves the ledger. Facts only move
    /// forward — provider retries and redeliveries are idempotent no-ops;
    /// unverified envelopes are refused.
    pub async fn payments_webhooks_ingest(&self, provider: String, id: Option<serde_json::Value>, request: Option<serde_json::Value>, verified: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/payments/webhooks/{provider}".replace("{provider}", &provider.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("provider".to_string(), serde_json::to_value(&provider)?);
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &request {
            api_params.insert("request".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &verified {
            api_params.insert("verified".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One ledger row in full: the amount and the fee that were computed at
    /// creation, the method code and PSP it was made through, where it stands in
    /// the lifecycle, the timestamp of each transition it has been through
    /// (`authorized_at`, `captured_at`, `failed_at`, `refunded_at`), the dunning
    /// columns the daily scan maintains and, while the buyer still has something
    /// to do, `next_action`. This is the call to poll after sending a buyer to a
    /// PSP redirect. Two things it does not do: `error_message` is answered from
    /// the failure taxonomy and never carries the provider's or the runtime's own
    /// words, and there is no route that resolves a payment by `order_ref` —
    /// that column is nullable and not unique, so it is a filter on the list (`GET
    /// /payments?order_ref=…`) which may legitimately answer several rows.
    pub async fn payments_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/payments/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Drops the claim before any money has been taken — the abandoned basket,
    /// the buyer who never came back from the redirect, the invoice an operator
    /// writes off. It is the only transition that starts from three statuses
    /// rather than one, because everything short of captured can still be
    /// released. A captured payment is not cancellable at all: that is a refund,
    /// and the lattice answers 400 rather than pretending. Unlike capture and
    /// refund this transition has no time window — the merchant's
    /// `capture_expiry_days` and `refund_window_days` do not apply, so a stale
    /// authorization can always be released even once it is too old to collect. On
    /// a PSP payment the provider is called and the `reason` in the body is passed
    /// to it, so it reaches the PSP's own cancellation-reason field as well as
    /// being stored under `metadata.cancel_reason`. Cancelling stops the dunning
    /// clock: the stage goes back to `none` and the due date is cleared.
    pub async fn payments_cancel(&self, id: String, reason: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/payments/{id}/cancel".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &reason {
            api_params.insert("reason".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Collects money that is currently only reserved. It starts from `authorized`
    /// and from nothing else — under `auto_capture_policy: 'immediate'` a
    /// payment is captured in the same request that created it and never passes
    /// through here, so this is the route for the 'manual' and 'on_ship' policies,
    /// and POST /payments/orders/{order_ref}/capture is the same operation
    /// addressed by the order reference a warehouse actually holds. There is no
    /// request body and no amount: the ledger carries one amount and one status,
    /// so a capture is the whole authorization or nothing. On a self-managed
    /// payment it takes no PSP anywhere near it — it records that an invoice or
    /// a prepayment was paid, and stops the dunning clock. Refused with 422 once
    /// the authorization is older than the tenant's `capture_expiry_days` (the
    /// message carries both numbers), because an expired authorization is declined
    /// by the provider anyway and a 422 here is the cheap version of finding out
    /// later.
    pub async fn payments_capture(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/payments/{id}/capture".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The other half of a redirect. POST /payments answered `requires_action`
    /// with a `next_action` the storefront carried out — a 3-D Secure step, a
    /// wallet approval, a bank login — and this is the call that asks the PSP
    /// how it went and writes the answer to the ledger. It starts from
    /// `requires_action` and from nothing else, so a payment that already came
    /// back authorized needs no confirm and the lattice answers 400 rather than
    /// repeating one. `next_action` is cleared by this call whatever the outcome.
    /// Where the tenant's `auto_capture_policy` is 'immediate' the money is taken
    /// straight after the authorization, in the same request, so a successful
    /// confirm can come back `captured` rather than `authorized`; a failed
    /// auto-capture does not fail the confirm, because a good authorization is
    /// worth more than a tidy status.
    pub async fn payments_confirm(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/payments/{id}/confirm".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Gives captured money back. It starts from `captured` and from nothing else
    /// — money that was only authorized is cancelled, not refunded, and the
    /// lattice answers 400 rather than guessing which was meant. All or nothing:
    /// the ledger carries one amount and one status, so there is no partial refund
    /// and no second one to express — a refunded payment is refunded in full,
    /// and a repeat is a 400 because `refunded` is not a status a refund starts
    /// from. The `reason` in the body is handed to the driver in the same call, so
    /// it reaches the PSP's own refund-reason field rather than being a note only
    /// this database ever sees, and it is stored under `metadata.refund_reason`.
    /// On a self-managed payment nothing is sent anywhere: it records that the
    /// merchant paid the buyer back by their own means. Refused with 422 once the
    /// capture is older than the tenant's `refund_window_days` (the message
    /// carries both numbers) — past that the provider stops accepting a refund
    /// against the transaction and it has to be made by bank transfer.
    pub async fn payments_refund(&self, id: String, reason: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/payments/{id}/refund".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &reason {
            api_params.insert("reason".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
