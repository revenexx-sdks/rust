use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Customers service
pub struct Customers {
    client: Client,
}

impl Customers {
    pub fn new(client: Client) -> Self {
        Customers { client }
    }
    /// An email and a password go in; a session and the CONTACT behind it come
    /// back, so a storefront knows in one call both that the buyer is signed in
    /// and who they are. The session is minted server-side rather than handed back
    /// from the credential check, because the account route hides the session
    /// secret from non-privileged responses and a trusted BFF needs it.
    /// `permissions` carries the buyer's effective grants, so a BFF does not need
    /// a second call to decide what to render.
    pub async fn customers_auth_login(&self, email: String, password: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/auth/login".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("email".to_string(), serde_json::to_value(&email)?);
        api_params.insert("password".to_string(), serde_json::to_value(&password)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Ends ONE session — the buyer signs out on this device and stays signed in
    /// on the others, because the session id is what is revoked and not the
    /// account. The contact row is untouched: signing out is not blocking, and a
    /// caller wanting the second thing wants `status: "blocked"` on the contact
    /// instead. Both ids come from what `/customers/auth/login` answered, and a
    /// BFF should drop its own cookie whatever this answers — the session is
    /// unusable afterwards either way.
    pub async fn customers_auth_logout(&self, session_id: String, user_id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/auth/logout".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("session_id".to_string(), serde_json::to_value(&session_id)?);
        api_params.insert("user_id".to_string(), serde_json::to_value(&user_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Sign in without a password: a link goes to the address, and `PUT
    /// /customers/auth/magic-link` turns it into a session. Creates the account
    /// when the address is new, which makes this a registration path as much as a
    /// sign-in one — and why an address nobody holds is not distinguished in the
    /// answer. The mail is this shop's own template through the messaging service;
    /// the secret is not in this response, only in the link.
    pub async fn customers_auth_magic_link(&self, email: String, url: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/auth/magic-link".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("email".to_string(), serde_json::to_value(&email)?);
        api_params.insert("url".to_string(), serde_json::to_value(&url)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The buyer clicked the link and the storefront read `userId` and `secret`
    /// out of it. Answers exactly what a password login answers — session,
    /// contact and effective grants — because a shop must not have to branch on
    /// how somebody signed in.
    pub async fn customers_auth_magic_link_confirm(&self, secret: String, user_id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/auth/magic-link".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("secret".to_string(), serde_json::to_value(&secret)?);
        api_params.insert("user_id".to_string(), serde_json::to_value(&user_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The platform user, the customer record mirrored against it and the
    /// effective grants, in one call. The expected caller is a trusted storefront
    /// BFF holding the session on the buyer's behalf, which is why the ids travel
    /// in the body rather than in a browser-facing header. The grants are derived
    /// here on every call rather than returned from anywhere they could be cached,
    /// so a role changed a second ago is already reflected.
    pub async fn customers_auth_me(&self, user_id: String, session_id: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/auth/me".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("user_id".to_string(), serde_json::to_value(&user_id)?);
        if let Some(value) = &session_id {
            api_params.insert("session_id".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Between the password and the finished session: the buyer has proved one
    /// thing and is asked for another. Created by user id, because the account
    /// route that creates challenges hides the code from whoever may call it —
    /// and answered with the half-finished session the sign-in is in the middle
    /// of, through `PUT /customers/auth/mfa/challenge`. Needs a platform build
    /// that returns the challenge code; without one there is no way to read what
    /// to send, and the call answers 502 rather than mailing an empty challenge.
    pub async fn customers_auth_mfa_challenge(&self, user_id: String, factor: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/auth/mfa/challenge".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("user_id".to_string(), serde_json::to_value(&user_id)?);
        if let Some(value) = &factor {
            api_params.insert("factor".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The code the buyer typed, against the challenge it was sent for. The
    /// session becomes fully authenticated when this answers.
    pub async fn customers_auth_mfa_challenge_confirm(&self, challenge_id: String, code: String, session_secret: String, user_id: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/auth/mfa/challenge".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("challenge_id".to_string(), serde_json::to_value(&challenge_id)?);
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("session_secret".to_string(), serde_json::to_value(&session_secret)?);
        if let Some(value) = &user_id {
            api_params.insert("user_id".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The same token as the sign-in link, delivered as a short code instead —
    /// for a buyer on a phone, where leaving for a mail client and coming back
    /// loses the checkout they were in the middle of. Redeemed with `PUT
    /// /customers/auth/otp`.
    pub async fn customers_auth_otp(&self, email: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/auth/otp".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("email".to_string(), serde_json::to_value(&email)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The code the buyer typed, plus the `userId` the send answered with. Answers
    /// exactly what a password login answers — session, contact and effective
    /// grants — so a storefront never has to branch on how somebody signed in.
    /// The code is spent on first use and expires, so a second attempt with the
    /// same one is a 401 rather than a second session.
    pub async fn customers_auth_otp_confirm(&self, secret: String, user_id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/auth/otp".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("secret".to_string(), serde_json::to_value(&secret)?);
        api_params.insert("user_id".to_string(), serde_json::to_value(&user_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Step one of two: a link goes to the address given, and `PUT
    /// /customers/auth/recovery` is what the buyer's browser comes back to. The
    /// identity service mints the token; the MAIL is this shop's own — the
    /// tenant's template, layout, language and sending domain, through the
    /// messaging service. The secret is NOT in this answer: it exists only inside
    /// the mailed link, which is the whole point of the two-step shape, and
    /// echoing it here would make the mail decorative. Nothing about the contact
    /// changes; the password only moves in step two.
    pub async fn customers_auth_recovery(&self, email: String, url: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/auth/recovery".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("email".to_string(), serde_json::to_value(&email)?);
        api_params.insert("url".to_string(), serde_json::to_value(&url)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Step two: the `userId` and `secret` the mailed link carried, plus the
    /// password the buyer just typed. The secret is spent on first use and
    /// expires, so a link cannot be replayed and a second attempt with the same
    /// one is a 401 rather than a second password change. The new password is in
    /// effect the moment this answers; what happens to sessions opened with the
    /// old one is the identity service's policy, not this app's.
    pub async fn customers_auth_recovery_confirm(&self, password: String, secret: String, user_id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/auth/recovery".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("password".to_string(), serde_json::to_value(&password)?);
        api_params.insert("secret".to_string(), serde_json::to_value(&secret)?);
        api_params.insert("user_id".to_string(), serde_json::to_value(&user_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One call writes the whole buyer: the contact this app is the system of
    /// record for, and the platform user behind its login. When the body names a
    /// company it also FOUNDS one — an organization, mirrored into platform auth
    /// as a team, with this contact as its admin. The tenant setting
    /// registration_mode decides what a registration IS. 'open' (the default,
    /// unchanged behaviour) creates a finished account:
    /// registration_status='approved', status='active', login works.
    /// 'approval_required' creates an APPLICATION: registration_status='pending',
    /// status='invited', the platform user exists with the applicant's own
    /// password but is DISABLED, and a newly founded organization is parked as
    /// 'blocked' — check `approval_required` in the response and show a 'we will
    /// get back to you' screen instead of logging the buyer in. The registration
    /// gates below are all evaluated BEFORE anything is written, and a failure
    /// after that point rolls the organization and the contact back together.
    pub async fn customers_auth_register(&self, email: String, password: String, first_name: Option<String>, last_name: Option<String>, locale: Option<String>, organization_id: Option<String>, organization_name: Option<String>, url: Option<String>, vat_id: Option<String>, verification_url: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/auth/register".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("email".to_string(), serde_json::to_value(&email)?);
        api_params.insert("password".to_string(), serde_json::to_value(&password)?);
        if let Some(value) = &first_name {
            api_params.insert("first_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &last_name {
            api_params.insert("last_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &locale {
            api_params.insert("locale".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_id {
            api_params.insert("organization_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_name {
            api_params.insert("organization_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &url {
            api_params.insert("url".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &vat_id {
            api_params.insert("vat_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &verification_url {
            api_params.insert("verification_url".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Confirm that the address belongs to the buyer. Needs no session: the
    /// verification is created through the identity service's users surface,
    /// because its account counterpart reads the authenticated user and a caller
    /// authenticating AS the user cannot see the secret it just created. The buyer
    /// still confirms with their own session, through `PUT
    /// /customers/auth/verification` — only the creation moved. Send it right
    /// after a registration, or from an account page.
    pub async fn customers_auth_verification(&self, url: String, user_id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/auth/verification".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("url".to_string(), serde_json::to_value(&url)?);
        api_params.insert("user_id".to_string(), serde_json::to_value(&user_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The `userId` and `secret` the mailed link carried. The address counts as
    /// confirmed the moment this answers; the secret is spent, so the link cannot
    /// be replayed.
    pub async fn customers_auth_verification_confirm(&self, secret: String, user_id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/auth/verification".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("secret".to_string(), serde_json::to_value(&secret)?);
        api_params.insert("user_id".to_string(), serde_json::to_value(&user_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The capability the API gateway calls to turn a caller's
    /// X-Revenexx-Principal assertion into the permission set it forwards to every
    /// other app as X-Revenexx-Permissions. This app is the platform's role
    /// provider (manifest#provides_roles), and this is the hot path of every
    /// attributed storefront request — one contact read plus the tenant's role
    /// map. A blocked or pending contact always resolves with active=false; what
    /// its `permissions` then say is the tenant's blocked_contact_behavior setting
    /// — 'keep' (the default, the role's grants), 'catalog_only' or 'deny_all'.
    pub async fn customers_principal_resolve(&self, contact_id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/principal/resolve".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("contact_id".to_string(), serde_json::to_value(&contact_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
