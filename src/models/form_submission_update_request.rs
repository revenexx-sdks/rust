use serde::{Deserialize, Serialize};

/// Partial update — typically the inbox status change.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormSubmissionUpdateRequest {
    /// What the visitor typed — the substance of the submission, and the reason
    /// this row is the payload of `form.submitted`.
    /// 
    /// It is an object keyed by the `name` of the definition node that collected
    /// each value, so the keys of a submission are the named nodes of its form's
    /// `definition` and nothing else. There is no fixed set of keys across forms:
    /// a contact form yields `{name, email, message}`, a price request whatever
    /// its operator built.
    /// 
    /// The VALUE type follows the input type, which is why this object is not
    /// typed further: a `text`, `email` or `textarea` yields a string, a `number`
    /// a number, a single `checkbox` a boolean, a `select`/`radio` the chosen
    /// option value, a multi-select or a checkbox set an array of them, and a
    /// `group` or `list` input nests an object or an array under its own name.
    /// Nothing coerces them — a value arrives as the storefront sent it and is
    /// stored as jsonb.
    /// 
    /// Two values are NOT here: the honeypot field, if the tenant configured one,
    /// is stripped before the row is written (it is a trap, not an answer the
    /// visitor gave), and the resolved notification recipient lives in `metadata`,
    /// not in what somebody typed.
    #[serde(rename = "data", default)]
    pub data: serde_json::Value,
    /// The form this submission was made against. It is resolved at insert, so an
    /// id no form in this tenant holds is a 404 and nothing is stored — a
    /// submission with no form is a lead nobody can read. Required on a create: it
    /// is the only thing that says which form was filled in.
    #[serde(rename = "form_id", default)]
    pub form_id: String,
    /// The form's slug as it stood when this submission arrived, copied onto the
    /// row: the inbox filters by form without a join, and a submission still says
    /// which form collected it after that form has been renamed. It does not
    /// outlive a DELETED form — the foreign key cascades and takes the
    /// submission with it. On a write the body's value WINS; omit it and the
    /// form's own slug is copied in. So: OPTIONAL — send it and it is stored as
    /// sent, even if it disagrees with the form; omit it and the form's own slug
    /// is filled in from `form_id`.
    #[serde(rename = "form_slug", default)]
    pub form_slug: String,
    /// Free-form metadata, yours to key as an integration needs. The resolved
    /// notification recipient is merged OVER it at insert, so `notify_email` and
    /// `notify_source` sent here are overwritten — see the
    /// `FormSubmissionMetadata` schema.
    #[serde(rename = "metadata", default)]
    pub metadata: serde_json::Value,
    /// Where the submission came from. The storefront sends the
    /// `window.location.pathname` of the page that carried the form, so this is
    /// normally a path rather than an absolute URL; any other surface (an app, an
    /// import) puts its own name here. Null when the caller sent none.
    #[serde(rename = "source", default)]
    pub source: String,
    /// Inbox triage. `new` until somebody opens it, then `read`, and `archived`
    /// once it is dealt with. `spam` is set by code in exactly one place — the
    /// honeypot, and only while the tenant's spam_handling is 'flag'; under
    /// 'reject' the submission is never stored at all. Default 'new'. A create may
    /// set it — an inbox importer records a submission that is already read —
    /// but nothing needs to: omit it and the row is 'new'.
    #[serde(rename = "status", default)]
    pub status: String,
}
