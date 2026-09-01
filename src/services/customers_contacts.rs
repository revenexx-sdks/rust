use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// CustomersContacts service
pub struct CustomersContacts {
    client: Client,
}

impl CustomersContacts {
    pub fn new(client: Client) -> Self {
        CustomersContacts { client }
    }
    /// A contact event is one entry on a customer's timeline: an activity somebody
    /// logged (a call, a visit, a meeting, a note) or a registration decision this
    /// app recorded itself. Every entry is keyed by a CONTACT and stamped with the
    /// organization derived from that contact, so a company's history is one
    /// indexed read rather than a join. Append-only — there is no update and no
    /// delete, which is what makes it usable as evidence. The activity feed,
    /// filtered by whichever column the question needs: `contact_id` for one
    /// person, `organization_id` for a whole company, `kind` for one type of
    /// activity. `kind: "system"` is this app's own registration decision trail
    /// (`registration.submitted` / `.approved` / `.rejected`), and no caller may
    /// file one of those. Paged with `limit`/`offset`/`order`; newest first is
    /// `order=occurred_at.desc`.
    pub async fn customers_contact_events_list(&self, id: Option<String>, contact_id: Option<String>, organization_id: Option<String>, kind: Option<String>, name: Option<String>, subject: Option<String>, actor: Option<String>, occurred_at: Option<String>, created_at: Option<String>, limit: Option<i64>, offset: Option<i64>, order: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/contact_events".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &contact_id {
            api_params.insert("contact_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_id {
            api_params.insert("organization_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &kind {
            api_params.insert("kind".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &subject {
            api_params.insert("subject".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &actor {
            api_params.insert("actor".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &occurred_at {
            api_params.insert("occurred_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &created_at {
            api_params.insert("created_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &offset {
            api_params.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order {
            api_params.insert("order".to_string(), serde_json::to_value(value)?);
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
    /// A contact event is one entry on a customer's timeline: an activity somebody
    /// logged (a call, a visit, a meeting, a note) or a registration decision this
    /// app recorded itself. Every entry is keyed by a CONTACT and stamped with the
    /// organization derived from that contact, so a company's history is one
    /// indexed read rather than a join. Append-only — there is no update and no
    /// delete, which is what makes it usable as evidence. One timeline entry by
    /// id, as it was written. Entries are never edited, so what this answers is
    /// what was recorded at the time.
    pub async fn customers_contact_events_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/contact_events/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A contact is a PERSON, and the unit that logs in: one platform user, one
    /// email address, one role held inside its organization. A contact without an
    /// organization is a standalone buyer rather than an error, and two people at
    /// the same company are two contacts sharing an `organization_id`. The people
    /// list, and the read behind an approval queue: `registration_status=pending`
    /// is every application waiting for a decision. Every column is a filter —
    /// `external_user_id` in particular is how a storefront turns a platform auth
    /// id back into a customer — and the page is `limit`/`offset`/`order`.
    pub async fn customers_contacts_list(&self, id: Option<String>, organization_id: Option<String>, email: Option<String>, first_name: Option<String>, last_name: Option<String>, phone: Option<String>, job_title: Option<String>, role: Option<String>, status: Option<String>, order_approval_limit: Option<f64>, registration_status: Option<String>, registration_decided_at: Option<String>, registration_decided_by: Option<String>, registration_reason: Option<String>, locale: Option<String>, is_primary: Option<bool>, external_user_id: Option<String>, created_at: Option<String>, updated_at: Option<String>, limit: Option<i64>, offset: Option<i64>, order: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/contacts".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_id {
            api_params.insert("organization_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &email {
            api_params.insert("email".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &first_name {
            api_params.insert("first_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &last_name {
            api_params.insert("last_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &phone {
            api_params.insert("phone".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &job_title {
            api_params.insert("job_title".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &role {
            api_params.insert("role".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order_approval_limit {
            api_params.insert("order_approval_limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &registration_status {
            api_params.insert("registration_status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &registration_decided_at {
            api_params.insert("registration_decided_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &registration_decided_by {
            api_params.insert("registration_decided_by".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &registration_reason {
            api_params.insert("registration_reason".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &locale {
            api_params.insert("locale".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_primary {
            api_params.insert("is_primary".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &external_user_id {
            api_params.insert("external_user_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &created_at {
            api_params.insert("created_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &updated_at {
            api_params.insert("updated_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &offset {
            api_params.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order {
            api_params.insert("order".to_string(), serde_json::to_value(value)?);
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
    /// A contact is a PERSON, and the unit that logs in: one platform user, one
    /// email address, one role held inside its organization. A contact without an
    /// organization is a standalone buyer rather than an error, and two people at
    /// the same company are two contacts sharing an `organization_id`. Creates the
    /// person and their platform login together, so a contact that exists can
    /// always sign in. `role` names one of this tenant's own roles and decides
    /// what they may do; `registration_status` may only be set to `pending` or
    /// `approved` here, because a rejection has to carry a reason and that is the
    /// reject route's job. `email` is the only field a create cannot omit;
    /// everything else is optional or defaulted by the database. Two rows of this
    /// tenant may not share `email` or `external_user_id` (while external_user_id
    /// IS NOT NULL).
    pub async fn customers_contacts_create(&self, email: String, first_name: Option<String>, is_primary: Option<bool>, job_title: Option<String>, last_name: Option<String>, locale: Option<String>, order_approval_limit: Option<f64>, organization_id: Option<String>, phone: Option<String>, registration_status: Option<String>, role: Option<String>, status: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/contacts".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("email".to_string(), serde_json::to_value(&email)?);
        if let Some(value) = &first_name {
            api_params.insert("first_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_primary {
            api_params.insert("is_primary".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &job_title {
            api_params.insert("job_title".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &last_name {
            api_params.insert("last_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &locale {
            api_params.insert("locale".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order_approval_limit {
            api_params.insert("order_approval_limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_id {
            api_params.insert("organization_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &phone {
            api_params.insert("phone".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &registration_status {
            api_params.insert("registration_status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &role {
            api_params.insert("role".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// This is how a call, a visit, a meeting, an email or a plain note reaches
    /// one person's timeline. It writes a contact_events row with kind != 'system'
    /// and emits contact_event.created, so an activity travels on the same bus as
    /// a registration decision and a timeline is one query rather than a union.
    /// organization_id is DERIVED from the contact, never taken from the body —
    /// an activity cannot be filed under a company the person does not belong to.
    pub async fn customers_contacts_events_create(&self, contact_id: String, subject: String, actor: Option<String>, kind: Option<String>, note: Option<String>, occurred_at: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/contacts/{contact_id}/events".replace("{contact_id}", &contact_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("contact_id".to_string(), serde_json::to_value(&contact_id)?);
        api_params.insert("subject".to_string(), serde_json::to_value(&subject)?);
        if let Some(value) = &actor {
            api_params.insert("actor".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &kind {
            api_params.insert("kind".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &note {
            api_params.insert("note".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &occurred_at {
            api_params.insert("occurred_at".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Tell somebody they were added to a company. A deliberate act rather than a
    /// side effect of creating the contact: a merchant entering a colleague from a
    /// business card is not always ready to mail them, and "added" and "told" are
    /// different decisions. No secret travels — the platform team membership is
    /// confirmed as it is created, so there is nothing to accept; the message says
    /// "you are in, here is the way in". Unlike the auth mails, a failure here IS
    /// a failure: the identity service sends nothing for this occasion, so this is
    /// the only message the person gets.
    pub async fn customers_contacts_invite(&self, contact_id: String, url: String, invited_by: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/contacts/{contact_id}/invite".replace("{contact_id}", &contact_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("contact_id".to_string(), serde_json::to_value(&contact_id)?);
        api_params.insert("url".to_string(), serde_json::to_value(&url)?);
        if let Some(value) = &invited_by {
            api_params.insert("invited_by".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Computed from contacts.role on every call — the grants are never
    /// persisted, so this always reflects the role the contact holds right now.
    pub async fn customers_contacts_permissions(&self, contact_id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/contacts/{contact_id}/permissions".replace("{contact_id}", &contact_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("contact_id".to_string(), serde_json::to_value(&contact_id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Only reachable for a contact whose registration_status is 'pending' or
    /// 'rejected' (approving a rejection reinstates it). Enables the platform user
    /// FIRST — the password the applicant chose at submit time works
    /// immediately, no new credential is issued — then sets
    /// registration_status='approved' and status='active', and un-blocks the
    /// organization this registration itself founded. Approving an
    /// already-approved registration is a no-op that emits nothing, so a retry is
    /// safe. Writes a contact_events row named 'registration.approved'.
    pub async fn customers_registrations_approve(&self, contact_id: String, decided_by: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/contacts/{contact_id}/registration/approve".replace("{contact_id}", &contact_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("contact_id".to_string(), serde_json::to_value(&contact_id)?);
        if let Some(value) = &decided_by {
            api_params.insert("decided_by".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Only reachable from 'pending'. Sets registration_status='rejected' and
    /// status='blocked', keeps the platform user in place but disabled — the
    /// email must not fall free for a silent second identity, and the merchant
    /// keeps the record. Delete the contact to remove both. 'reason' is mandatory
    /// and is stored on the contact plus carried in the event payload, so the
    /// applicant can be told why. Rejecting an already-rejected registration is a
    /// no-op. Writes a contact_events row named 'registration.rejected'.
    pub async fn customers_registrations_reject(&self, contact_id: String, reason: String, decided_by: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/contacts/{contact_id}/registration/reject".replace("{contact_id}", &contact_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("contact_id".to_string(), serde_json::to_value(&contact_id)?);
        api_params.insert("reason".to_string(), serde_json::to_value(&reason)?);
        if let Some(value) = &decided_by {
            api_params.insert("decided_by".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A contact is a PERSON, and the unit that logs in: one platform user, one
    /// email address, one role held inside its organization. A contact without an
    /// organization is a standalone buyer rather than an error, and two people at
    /// the same company are two contacts sharing an `organization_id`. Removes the
    /// person and their platform login, so they can no longer sign in anywhere.
    /// Their company keeps trading; use `status: "blocked"` instead when the
    /// intent is to stop one person without erasing what they did. Deleting one
    /// takes every `contact_events` and `addresses` row that points at it with it
    /// — the foreign keys decide, not this route.
    pub async fn customers_contacts_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/contacts/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A contact is a PERSON, and the unit that logs in: one platform user, one
    /// email address, one role held inside its organization. A contact without an
    /// organization is a standalone buyer rather than an error, and two people at
    /// the same company are two contacts sharing an `organization_id`. One person
    /// by id. What they are ALLOWED to do is not in here: permissions are derived
    /// from `role` at read time and answered by `GET
    /// /customers/contacts/{contact_id}/permissions`.
    pub async fn customers_contacts_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/contacts/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A contact is a PERSON, and the unit that logs in: one platform user, one
    /// email address, one role held inside its organization. A contact without an
    /// organization is a standalone buyer rather than an error, and two people at
    /// the same company are two contacts sharing an `organization_id`. A partial
    /// update — send only what changes. `external_user_id` and every
    /// `registration_*` column are ignored: the link to platform auth is
    /// mirror-managed, and registration state is only ever moved by the approve
    /// and reject routes, which record why. Two rows of this tenant may not share
    /// `email` or `external_user_id` (while external_user_id IS NOT NULL).
    pub async fn customers_contacts_update(&self, id: String, email: Option<String>, first_name: Option<String>, is_primary: Option<bool>, job_title: Option<String>, last_name: Option<String>, locale: Option<String>, order_approval_limit: Option<f64>, organization_id: Option<String>, phone: Option<String>, registration_status: Option<String>, role: Option<String>, status: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/contacts/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &email {
            api_params.insert("email".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &first_name {
            api_params.insert("first_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_primary {
            api_params.insert("is_primary".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &job_title {
            api_params.insert("job_title".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &last_name {
            api_params.insert("last_name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &locale {
            api_params.insert("locale".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order_approval_limit {
            api_params.insert("order_approval_limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_id {
            api_params.insert("organization_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &phone {
            api_params.insert("phone".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &registration_status {
            api_params.insert("registration_status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &role {
            api_params.insert("role".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Same row as the contact route, reached from the organization. 'contact_id'
    /// is required and must belong to THIS organization — the picker offering
    /// the contacts is not filtered, so the membership check here is what stops a
    /// call with one company being filed under someone else's person.
    pub async fn customers_organizations_events_create(&self, organization_id: String, contact_id: String, subject: String, actor: Option<String>, kind: Option<String>, note: Option<String>, occurred_at: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/organizations/{organization_id}/events".replace("{organization_id}", &organization_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("organization_id".to_string(), serde_json::to_value(&organization_id)?);
        api_params.insert("contact_id".to_string(), serde_json::to_value(&contact_id)?);
        api_params.insert("subject".to_string(), serde_json::to_value(&subject)?);
        if let Some(value) = &actor {
            api_params.insert("actor".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &kind {
            api_params.insert("kind".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &note {
            api_params.insert("note".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &occurred_at {
            api_params.insert("occurred_at".to_string(), serde_json::to_value(value)?);
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
