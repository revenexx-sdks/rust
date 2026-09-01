use serde::{Deserialize, Serialize};

/// Creates the contact (system of record) and mirrors it as a platform user
/// (status defaults to invited).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContactCreateRequest {
    /// Login identity and the unique key of a person within the tenant. Changing
    /// it changes the platform login with it. Two people at the same company
    /// therefore need two addresses — a shared purchasing mailbox is one
    /// contact, not several.
    #[serde(rename = "email", default)]
    pub email: String,
    /// Given name. Optional: an ERP import often has only a mailbox.
    #[serde(rename = "first_name", default)]
    pub first_name: String,
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
    /// organization sets this null and keeps the person. Membership is mirrored to
    /// the platform team.
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    /// Direct number of this person, as somebody typed it — free text, no format
    /// is enforced or normalized. E.164 is what an integration should send.
    #[serde(rename = "phone", default)]
    pub phone: String,
    /// Where this person's own application stands: 'approved' (the default, and
    /// what an open store creates), 'pending' while a merchant has yet to decide,
    /// 'rejected' once they declined. Only the approve/reject routes move it; it
    /// is ignored on an ordinary update. On CREATE only, and only to file the
    /// contact as an application: 'pending' creates the platform user disabled and
    /// routes the contact through approve/reject. Ignored on update.
    #[serde(rename = "registration_status", default)]
    pub registration_status: String,
    /// The person's role INSIDE its organization, and the only thing permissions
    /// are derived from. One of the tenant's own roles (GET /customers/roles); a
    /// tenant that never edited the ledger has viewer, requester, buyer, approver,
    /// admin. Also the team role on the platform mirror. There is no global role
    /// — the same person in two companies is two contacts. A tenant that never
    /// edited the ledger has viewer, requester, buyer, approver, admin; a create
    /// without a role gets the one flagged as default, and a role the tenant does
    /// not keep is a 400.
    #[serde(rename = "role", default)]
    pub role: String,
    /// Whether this person may act: 'invited' has been created but has not
    /// accepted, 'active' works, 'blocked' cannot log in. A create through the API
    /// defaults to 'invited'; a self-registration in an open store lands 'active'.
    /// Default 'invited' on create.
    #[serde(rename = "status", default)]
    pub status: String,
}
