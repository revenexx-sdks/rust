use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthRegisterRequest {
    /// The buyer's address. It becomes the login AND the unique key of the
    /// contact, so a second registration with it is a 409 — including while the
    /// first one is still waiting for approval.
    #[serde(rename = "email", default)]
    pub email: String,
    /// Given name. Optional: an ERP import often has only a mailbox.
    #[serde(rename = "first_name", default)]
    pub first_name: String,
    /// Family name. Optional for the same reason.
    #[serde(rename = "last_name", default)]
    pub last_name: String,
    /// The language this person is written to in — BCP 47, and one of the
    /// store's configured locales. Null falls back to the store default. One of
    /// the store's own locales, or the call is a 400.
    #[serde(rename = "locale", default)]
    pub locale: String,
    /// JOIN an existing company — the invite shape. Neither
    /// b2b_registration_enabled nor b2c_registration_enabled applies to it.
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    /// FOUND a new company, with this contact as its admin. This is what makes the
    /// registration a B2B one; leaving it out registers a standalone buyer.
    #[serde(rename = "organization_name", default)]
    pub organization_name: String,
    /// The password the buyer chooses. It is hashed by the identity service at
    /// this moment and never travels again: an approval later enables the account,
    /// it does not issue a new credential.
    #[serde(rename = "password", default)]
    pub password: String,
    /// Where the welcome mail's button points — the buyer's first stop in this
    /// shop. Absent, the mail still goes out and simply carries no button. Ignored
    /// when the registration is an APPLICATION: there is no account to send
    /// anybody to yet.
    #[serde(rename = "url", default)]
    pub url: String,
    /// VAT identification number (USt-IdNr. in Germany) — the closest thing a
    /// B2B buyer has to a legal identity. Validated against the EU VIES service
    /// when the tenant's `organization_vat_id_required` setting is on, and stored
    /// verbatim otherwise, including for buyers outside the EU. Required when the
    /// tenant's `organization_vat_id_required` is on, and checked BEFORE the
    /// company is created so a bad one leaves no half-founded organization behind.
    #[serde(rename = "vat_id", default)]
    pub vat_id: String,
    /// Where the address-confirmation link points, when the tenant's
    /// `email_verification` asks for one on registration. `userId`, `secret` and
    /// `expire` are appended, and `PUT /customers/auth/verification` takes the
    /// first two. Without it the registration still succeeds and
    /// `verification_sent` is false — this app cannot invent a storefront URL,
    /// and a link pointing nowhere is worse than none.
    #[serde(rename = "verification_url", default)]
    pub verification_url: String,
}
