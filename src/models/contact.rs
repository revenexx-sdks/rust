use serde::{Deserialize, Serialize};

/// A PERSON, and the unit that logs in: one platform user, one email, one role
/// inside its organization. A contact without an organization is a standalone
/// buyer, not an error.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Contact {
    /// When this person record was created in this app.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
    /// Login identity and the unique key of a person within the tenant. Changing
    /// it changes the platform login with it. Two people at the same company
    /// therefore need two addresses — a shared purchasing mailbox is one
    /// contact, not several.
    #[serde(rename = "email", default)]
    pub email: String,
    /// Id of the platform USER this contact is mirrored as — the account that
    /// actually holds the password and the sessions. Written by the mirror and
    /// ignored on every write a caller sends.
    #[serde(rename = "external_user_id", default)]
    pub external_user_id: String,
    /// Given name. Optional: an ERP import often has only a mailbox.
    #[serde(rename = "first_name", default)]
    pub first_name: String,
    /// Primary key of the person record. What the timeline, the permission routes
    /// and the principal resolution all name.
    #[serde(rename = "id", default)]
    pub id: String,
    /// The main contact of its organization — who a merchant calls first. At
    /// most one per company is the intent; the tenant's `primary_contact_required`
    /// setting decides whether the last one may be demoted or deleted.
    #[serde(rename = "is_primary", default)]
    pub is_primary: bool,
    /// What this person does at the company — free text on purpose, because it
    /// is a title and not a grant. The permission ladder is `role`; overloading a
    /// job title with authority silently un-grants everyone the day the ledger is
    /// enforced.
    #[serde(rename = "job_title", default)]
    pub job_title: String,
    /// Family name. Optional for the same reason.
    #[serde(rename = "last_name", default)]
    pub last_name: String,
    /// The language this person is written to in — BCP 47, and one of the
    /// store's configured locales. Null falls back to the store default.
    #[serde(rename = "locale", default)]
    pub locale: String,
    /// Amount ceiling for this person, in the market's currency: with the
    /// `orders.approve` permission it is the most they may sign off. Null means no
    /// ceiling. An amount, never a grant — the grant comes from the role.
    #[serde(rename = "order_approval_limit", default)]
    pub order_approval_limit: f64,
    /// The company this person belongs to. NULL is a legitimate state, not a
    /// defect: a standalone buyer with no company behind them. Deleting the
    /// organization sets this null and keeps the person.
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    /// Direct number of this person, as somebody typed it — free text, no format
    /// is enforced or normalized. E.164 is what an integration should send.
    #[serde(rename = "phone", default)]
    pub phone: String,
    /// When a merchant approved or rejected the application. Null while nobody has
    /// decided.
    #[serde(rename = "registration_decided_at", default)]
    pub registration_decided_at: String,
    /// Who decided — free text as the deciding client supplied it (an operator
    /// id or an email address), not a resolvable user reference.
    #[serde(rename = "registration_decided_by", default)]
    pub registration_decided_by: String,
    /// Why the application was declined. Always recorded here; whether the
    /// APPLICANT is ever told it is the tenant's `registration_reason_disclosed`
    /// setting, because that is a legal decision and not a template one.
    #[serde(rename = "registration_reason", default)]
    pub registration_reason: String,
    /// Where this person's own application stands: 'approved' (the default, and
    /// what an open store creates), 'pending' while a merchant has yet to decide,
    /// 'rejected' once they declined. Only the approve/reject routes move it; it
    /// is ignored on an ordinary update.
    #[serde(rename = "registration_status", default)]
    pub registration_status: String,
    /// The person's role INSIDE its organization, and the only thing permissions
    /// are derived from. One of the tenant's own roles (GET /customers/roles); a
    /// tenant that never edited the ledger has viewer, requester, buyer, approver,
    /// admin. Also the team role on the platform mirror. There is no global role
    /// — the same person in two companies is two contacts.
    #[serde(rename = "role", default)]
    pub role: String,
    /// Whether this person may act: 'invited' has been created but has not
    /// accepted, 'active' works, 'blocked' cannot log in. A create through the API
    /// defaults to 'invited'; a self-registration in an open store lands 'active'.
    #[serde(rename = "status", default)]
    pub status: String,
    /// The tenant this row belongs to — the store slug, not an id. Set by the
    /// platform from the authenticated context, never by a caller; a write that
    /// carries it is ignored, and no request can read another tenant's rows by
    /// sending a different one.
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: String,
    /// When any column of this row last changed.
    #[serde(rename = "updated_at", default)]
    pub updated_at: String,
}
