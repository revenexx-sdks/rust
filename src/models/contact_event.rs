use serde::{Deserialize, Serialize};

/// One entry on a customer's timeline: an activity somebody logged (call,
/// visit, note) or a registration decision this app recorded. Append-only —
/// nothing here is ever edited.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContactEvent {
    /// Who logged the entry — free text as the client supplied it (operator id
    /// or email). Null for a row the app wrote itself.
    #[serde(rename = "actor", default)]
    pub actor: String,
    /// The person this entry is about. Always set: even a company-level activity
    /// is filed against somebody, so a timeline never has anonymous rows.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// When the row was written. Together with `occurred_at` this is what tells a
    /// late entry from a live one.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// Primary key of the timeline entry.
    #[serde(rename = "id", default)]
    pub id: String,
    /// What kind of entry this is — one of the tenant's own activity types (GET
    /// /customers/contact-event-kinds), seeded with note, call, email, meeting,
    /// visit, task. 'system' is reserved: those rows are this app's own
    /// registration decision trail and no caller may file one.
    #[serde(rename = "kind", default)]
    pub kind: String,
    /// The event name, and the one vocabulary here that is THIS APP's rather than
    /// the tenant's: `registration.submitted` | `registration.approved` |
    /// `registration.rejected` for decisions, `activity.<kind>` for everything
    /// somebody logged. It is also what travels on the bus as
    /// `contact_event.created`.
    #[serde(rename = "name", default)]
    pub name: String,
    /// When the thing actually HAPPENED, which is not when it was written down: a
    /// call logged on Monday about Friday says Friday. Defaults to now.
    #[serde(rename = "occurred_at", default)]
    pub occurred_at: String,
    /// The company this entry belongs to, DERIVED from the contact and never taken
    /// from a request body — which is what stops a call with one company being
    /// filed under someone else's person. Null when the contact has no
    /// organization.
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    /// The machine-readable body, and its shape follows `name`. `activity.<kind>`
    /// carries `{ note }` — the long form of `subject`. `registration.submitted`
    /// carries the application itself: email, organization_id, organization_name,
    /// role, locale, vat_id, and `notify`, the recipients the approval mail goes
    /// to. `registration.approved` carries `{ decided_by }`;
    /// `registration.rejected` adds `reason`. Nothing validates it beyond that —
    /// a client writing its own entries decides what belongs in here.
    #[serde(rename = "payload", default)]
    pub payload: serde_json::Value,
    /// One line a person can scan in a timeline. Required for an activity; a
    /// decision row carries the app's own wording.
    #[serde(rename = "subject", default)]
    pub subject: String,
    /// The tenant this row belongs to — the store slug, not an id. Set by the
    /// platform from the authenticated context, never by a caller; a write that
    /// carries it is ignored, and no request can read another tenant's rows by
    /// sending a different one.
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
}
