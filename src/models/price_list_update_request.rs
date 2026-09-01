use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PriceListUpdateRequest {
    /// Scope: only this sales channel. Beats the open lists, loses to contact and
    /// organization.
    #[serde(rename = "channel_id", default)]
    pub channel_id: String,
    /// Unique list code per tenant — the handle every import and integration
    /// addresses this list by. A code already in use answers 409.
    #[serde(rename = "code", default)]
    pub code: String,
    /// Scope: only this contact. The most specific scope there is — it beats
    /// organization, channel and every open list, whatever their priority.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// ISO 4217 code (default EUR) — the currency of EVERY amount in this list,
    /// since entries carry none of their own. Resolution only considers lists
    /// matching the currency of the call; nothing is ever converted.
    #[serde(rename = "currency", default)]
    pub currency: String,
    /// Free text for whoever maintains the list — why it exists and who it is
    /// for. Never shown to a buyer.
    #[serde(rename = "description", default)]
    pub description: String,
    /// The fallback list. Within its group it sorts LAST, so it wins only where
    /// nothing more specific priced the item. Use prices.lists.make-default to
    /// move the flag rather than setting it here — two defaults leave a tie to
    /// row order.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Localised names, keyed by language tag — {"de": "Händlerpreise", "en":
    /// "Dealer prices"}. Omit to show `name` everywhere.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// Free-form bag: whatever JSON object you write round-trips exactly, and this
    /// app never reads it. Its keys are yours — ERP provenance is the usual
    /// content.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// Operator-facing name, shown wherever a human picks a list.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Scope: only buyers of this organization. Beats channel-scoped and open
    /// lists.
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    /// Tie-break WITHIN a specificity group (higher wins, default 0). It never
    /// beats scope: an organization list at 0 still wins over an open list at 100.
    #[serde(rename = "priority", default)]
    pub priority: i64,
    /// Gate: when true the list resolves only for an authenticated buyer (contact
    /// or organization context); anonymous resolve calls get on_request. Default
    /// false (open to everyone).
    #[serde(rename = "requires_auth", default)]
    pub requires_auth: bool,
    /// Default 'active' — only active lists resolve. 'inactive' retires a list
    /// without deleting its prices.
    #[serde(rename = "status", default)]
    pub status: String,
    /// Whether the amounts in this list are net (tax excluded) or gross (tax
    /// included) — the one fact a price cannot be without. Omit (null) to
    /// inherit the tenant's tax_inclusive_default setting; the resolve answer
    /// names which of the two decided under tax_basis_source.
    #[serde(rename = "tax_basis", default)]
    pub tax_basis: String,
    /// LEGACY mirror of tax_basis. false is the column default and is NOT read as
    /// a statement of intent; true is read as gross, and only where tax_basis is
    /// null. Prefer tax_basis.
    #[serde(rename = "tax_included", default)]
    pub tax_included: bool,
    /// Start of the validity window of the WHOLE list (ISO 8601); null =
    /// open-ended. Outside it the list is not a candidate at all.
    #[serde(rename = "valid_from", default)]
    pub valid_from: String,
    /// End of the validity window of the whole list; null = open-ended. Lets a
    /// season expire on its own instead of being deactivated by hand.
    #[serde(rename = "valid_until", default)]
    pub valid_until: String,
}
