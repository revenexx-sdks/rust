use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Channels service
pub struct Channels {
    client: Client,
}

impl Channels {
    pub fn new(client: Client) -> Self {
        Channels { client }
    }
    /// The filters are what make this list usable: `?code=` turns a scope slug
    /// another app stored into the channel row that owns it, `?is_default=true`
    /// finds the fallback channel without resolving a context, and
    /// `?unassigned_visibility=assigned_only` finds the channels that closed their
    /// assortment. Every filter is an exact-column equality — there is no
    /// contains, prefix or range form — and the honoured set is exactly this
    /// entity's 11 columns, because the generic list mount matches any query key
    /// that names one. Each of them is declared as a query parameter with the
    /// column's own CHECK behind it, so the 11 that work are the 11 the document
    /// offers rather than a list a caller has to keep somewhere. An unknown column
    /// is dropped rather than refused, so `?stauts=active` returns the unfiltered
    /// page; `filter` echoes what was understood, which is the only way to catch
    /// that. Paging is `limit`/`offset` over whatever survived the filters, and
    /// `?order=` sorts by one column with an optional `.asc`/`.desc`; ask for no
    /// order and the page comes back in insertion order. `order` is the one input
    /// here that is refused rather than ignored — a malformed value, or one
    /// naming a column this entity does not have, is a 400 where the same mistake
    /// in a filter key passes silently.
    pub async fn channels_list(&self, id: Option<String>, code: Option<String>, name: Option<String>, labels: Option<String>, xtype: Option<String>, status: Option<String>, unassigned_visibility: Option<String>, is_default: Option<bool>, position: Option<i64>, created_at: Option<String>, updated_at: Option<String>, limit: Option<i64>, offset: Option<i64>, order: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/channels".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &xtype {
            api_params.insert("type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unassigned_visibility {
            api_params.insert("unassigned_visibility".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
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

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Two fields are yours and everything else has an answer already: `code` and
    /// `name` are the only columns the database will not fill in, and the rest
    /// arrive from their defaults — `status` active, `unassigned_visibility`
    /// inherit, `is_default` false, `position` 0. `type` is the exception the app
    /// makes for itself: omitted, it becomes the type the tenant FLAGGED as their
    /// default rather than the column default, so a merchant who retired the
    /// seeded `storefront` does not get channels carrying a type they no longer
    /// keep. `code` is the load-bearing one. It is the scope slug Baseline matches
    /// every channel assignment on, which is why it is held to Baseline's own
    /// shape here rather than to the column's `length > 0`, and why it is unique
    /// per tenant — a second channel claiming a code another already holds is a
    /// 409 off the `(tenant_id, code)` index. Treat it as permanent: the API will
    /// let you change it later and nothing follows it (see PUT /channels/{id}).
    /// Creating a channel assigns nothing to it. Products, categories and
    /// everything else scopeable stay exactly as visible as they were — until
    /// rows are assigned, what this channel shows is whatever
    /// `unassigned_channel_visibility` says, which on the shipped default is the
    /// entire catalogue. And a code is only free in THIS app: assignments made
    /// against a code that a since-deleted channel used are still in Baseline, so
    /// re-using the code adopts them.
    pub async fn channels_create(&self, code: String, name: String, is_default: Option<bool>, labels: Option<serde_json::Value>, position: Option<i64>, status: Option<String>, xtype: Option<String>, unassigned_visibility: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/channels".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &xtype {
            api_params.insert("type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unassigned_visibility {
            api_params.insert("unassigned_visibility".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The storefront/punchout bootstrap: one call tells a shop front, a punchout
    /// front-end or a feed builder which channel it is in and what an unassigned
    /// row means there, so it can apply the policy itself instead of hardcoding
    /// one. Resolution order is body/query, then the x-revenexx-channel header,
    /// then the scope_context.channel claim, then the channel flagged is_default
    /// — header before claim, the same order baseline.is_visible() uses. Through
    /// api.revenexx.com the header step is inert (the gateway does not forward
    /// it), so in practice it is `?channel=`, then the claim, then the default.
    /// Never errors on an unknown or inactive channel: it answers resolved:false
    /// with a reason, so a caller can tell "no such channel" from "the service is
    /// down". That is why this operation declares no 4xx of its own — a tenant
    /// with no channels at all answers 200 with reason no_default_channel. Two
    /// things come back, not one: the channel that was resolved, and the
    /// visibility policy in force for it — the tenant-wide
    /// unassigned_channel_visibility answer, or the channel's own override where
    /// it has one. The policy travels with the channel because a caller that has
    /// one and not the other still cannot render anything: knowing you are in the
    /// punchout channel says nothing about whether an unassigned product belongs
    /// in its catalogue. With both, a client reproduces the decision itself and
    /// calls POST /channels/visibility only when it wants the app to decide row by
    /// row.
    pub async fn channels_context(&self, channel: Option<String>) -> Result<crate::models::ChannelContext, Error> {
        let api_path = "/v1/channels/context".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &channel {
            api_params.insert("channel".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The repair call. A tenant installed before `channel_types` existed, or one
    /// that deleted its way into a state where nothing works, gets the shipped
    /// starting point back — the 5 seeded types first, because the seeded
    /// channel carries one of them, then the `shop` channel. Most tenants never
    /// call it: the platform invokes the same routine itself on `app.installed`,
    /// so a fresh install already has its 5 types and its shop channel before
    /// anyone asks, and this route exists for the tenant whose install predates
    /// them or who has since removed them. Calling it on a settled tenant is safe
    /// and cheap for the same reason it is safe to fire on every install: it is
    /// idempotent, keyed on the code, so a second call writes nothing. Everything
    /// a merchant added themselves is left alone, and a row that already exists is
    /// reported under `existing` rather than rewritten — the values you edited
    /// on a seeded type survive this call. It RESTORES THE WHOLE SEED SET,
    /// including a seeded type the merchant deliberately deleted. Idempotency here
    /// is keyed on the code and nothing else, and there is nowhere to remember a
    /// retirement: retirement is not a state this app can represent. Retiring a
    /// type IS deleting the row; `channel_types` has no retired flag and these
    /// tables carry no foreign keys, so nothing anywhere distinguishes a code a
    /// merchant removed on purpose from one they never had. Honouring the
    /// retirement would mean inventing a tombstone rather than reading one. Given
    /// that, restoring all 5 is the better half of the trade: this is the call a
    /// tenant makes when something is missing, and a repair that silently skips
    /// part of what it repairs, with no way to ask for the rest, is worse than one
    /// that says plainly what it puts back. It is also never a surprise. The only
    /// automatic seeding elsewhere in the app fires when the type table is
    /// completely EMPTY, which cannot happen once installed because the last
    /// remaining type cannot be deleted — so a retired type comes back exactly
    /// when somebody calls this route or the app is installed again, and never as
    /// a side effect of an unrelated read. Deleting it a second time costs one
    /// DELETE, and is refused only if a channel has since started carrying it.
    /// What it does not do: it creates no assignments, it does not repair a
    /// channel whose own code you deleted (only `shop` comes back), and it does
    /// not restore the seeded VALUES of a type that still exists — a renamed
    /// `storefront` stays renamed.
    pub async fn channels_defaults(&self) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/channels/defaults".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// What a channel may BE. This used to be a CHECK constraint over five values,
    /// which meant the merchant who runs a feed channel or a print channel needed
    /// a release of this app to say so — and nothing in the app ever branched on
    /// the value, only on membership. The set is the tenant's rows now. Seeds
    /// itself on first read, so the list is never empty and a channel can always
    /// carry a type. Rows come back in `position` order, always: this route is not
    /// the generic list mount and takes no `order` — `limit` and `offset` are
    /// the whole of its query, and it takes no filters, so a caller looking for
    /// one code reads the list and matches. The set is bounded: a tenant keeps at
    /// most 200 types, which is the size this app can check a channel's type
    /// against in one query, and POST /channels/types refuses the 201st rather
    /// than build a set it could not read back. `page.total` counts the rows that
    /// exist, not the ones this answer carries, and the order is total —
    /// `position` then `code`, because `position` is not unique and an order that
    /// leaves rows tied is one the database is free to answer differently on the
    /// next page, which is how a walk serves a row twice and skips another.
    pub async fn channels_types_list(&self, limit: Option<i64>, offset: Option<i64>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/channels/types".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &offset {
            api_params.insert("offset".to_string(), serde_json::to_value(value)?);
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
    /// What lets a merchant name a kind of channel this app never thought of — a
    /// feed, a print catalogue, a kiosk — without waiting for a release. `code`
    /// and `title` are the only two the database will not fill in; everything else
    /// has a default. The code is trimmed and lowercased and becomes exactly what
    /// `channels.type` stores, and it is fixed from then on, because there is no
    /// foreign key behind that column to carry a rename: every channel holding the
    /// old string would be left pointing at nothing. The title is the part a
    /// merchant renames later. A duplicate code is a 409, and it is worth knowing
    /// that the collision is wider than this tenant — `channel_types.code` is
    /// unique on the column alone, so a code held by another tenant collides too
    /// and the read this route does before inserting cannot see it. A tenant keeps
    /// at most 200 types; the 201st is a 409 `type_limit_reached` rather than a
    /// row the app would then be unable to read back. Creating a type changes
    /// nothing about existing channels: it is a name that becomes available, not
    /// one that gets applied. Adding a type does not make it the default either
    /// — pass `is_default: true` for that, which demotes the current holder.
    pub async fn channels_types_create(&self, code: String, title: String, description: Option<String>, descriptions: Option<serde_json::Value>, is_default: Option<bool>, labels: Option<serde_json::Value>, position: Option<i64>, tone: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/channels/types".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("title".to_string(), serde_json::to_value(&title)?);
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &descriptions {
            api_params.insert("descriptions".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tone {
            api_params.insert("tone".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Retiring a type IS deleting the row — there is no retired flag on
    /// `channel_types` — which is why the two things that would make a deletion
    /// destructive are refused instead of allowed: a type at least one channel
    /// still carries is a 409, and so is the last remaining type. There is no
    /// foreign key behind `channels.type`, so those two checks are not a
    /// convenience on top of the database, they ARE the integrity. Move the
    /// channels to another type first and the delete goes through. Nothing else
    /// goes with it. A type has no dependents once no channel names it: no rows in
    /// this app point at it and none in Baseline do either, since assignments are
    /// made against a channel `code`, never a type. Deleting the type the tenant
    /// had flagged as default is allowed, and the flag is handed to the next type
    /// by position rather than left unheld, so a channel created afterwards still
    /// has something to fall back to. Because the guard is a read followed by a
    /// write with no transaction between them, and no constraint underneath it, a
    /// channel created against this type in the same instant can survive it. Worth
    /// knowing what that leaves, since it is not what "orphaned" usually means:
    /// the channel keeps working. `channels.type` is a stored string that nothing
    /// joins on, so the channel still reads, still filters under `?type=` by that
    /// same string, and still resolves in /channels/context and POST
    /// /channels/visibility — neither of which consults `type` at all. What it
    /// loses is its label, because the types vocabulary is built from the rows and
    /// there is no longer one to render a badge from. An update that does not
    /// mention `type` leaves the value alone; naming it is refused, which is how
    /// the channel is moved to a type that exists. One thing the deletion frees is
    /// wider than the tenant: `channel_types.code` is unique on the column alone,
    /// so the code becomes available platform-wide, not just here. And the seed
    /// does not know the row is gone — POST /channels/defaults and a re-install
    /// both put a deleted SEEDED type back, by design; see that operation.
    pub async fn channels_types_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/channels/types/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One type row, by its uuid — the handle PUT and DELETE take, and the
    /// reason to hold on to what the list gave you. It is NOT the `code`: the code
    /// is what `channels.type` stores, and this route will not look one up.
    /// Neither will the list, which takes no filters at all, so a caller holding
    /// only a code pages `GET /channels/types` and matches client-side. Since the
    /// whole set is bounded and small that is one call, not a search. Unlike the
    /// list, this route does not seed. The list is hand-written so that a tenant
    /// whose table is still empty is given the 5 shipped types instead of being
    /// told they have none; this is the generic item route, so on that same tenant
    /// it answers 404 for every id — which is the correct answer, since there is
    /// genuinely no such row yet. Read the list first. Nothing here is cached: the
    /// type list changes when a merchant edits it and this route always reflects
    /// that. Rows seeded before 0.7.0 may hold a serialized locale map in `title`
    /// or `description` rather than plain text (PE-452); `labels` and
    /// `descriptions` are the columns that carry the per-locale copy now.
    pub async fn channels_types_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/channels/types/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Everything but the code. This is where a merchant renames a seeded type
    /// into their own words, gives it its German, moves it in the list a person
    /// picks from, or hands it the default flag. Seeded types are as editable as
    /// ones the merchant added — `is_system` records where a row came from and
    /// grants it nothing. Sending a different `code` is a 400 rather than a silent
    /// no-op: it is what `channels.type` stores, there is no foreign key behind
    /// that column to carry the change — the database has none at all on these
    /// tables — and a rename would therefore move nothing. Every channel holding
    /// the old string would keep holding it, still working but with no type row to
    /// draw its name from. This refusal is the whole of the protection; to move
    /// channels to a new code, create the type and update the channels, in that
    /// order. Two fields are quietly forgiving rather than strict — a blank
    /// `title` and a `tone` outside the palette are both ignored and the stored
    /// value kept, so a client that sends a half-filled form does not clear what
    /// is there. `is_default` is one-way: true promotes this type and demotes the
    /// previous holder, false does nothing at all, because some type has to be the
    /// one a channel created without one gets.
    pub async fn channels_types_update(&self, id: String, description: Option<String>, descriptions: Option<serde_json::Value>, is_default: Option<bool>, labels: Option<serde_json::Value>, position: Option<i64>, title: Option<String>, tone: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/channels/types/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &description {
            api_params.insert("description".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &descriptions {
            api_params.insert("descriptions".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &title {
            api_params.insert("title".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tone {
            api_params.insert("tone".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The gate. A row WITH channel assignments is decided exactly as
    /// baseline.is_visible() decides it — visible iff the active channel is
    /// among them. A row WITHOUT assignments is the case
    /// unassigned_channel_visibility owns: 'all' shows it (Baseline's
    /// open-by-default, unchanged) and 'assigned_only' hides it, which the
    /// generated _scoped view has no way to express. A channel may override the
    /// tenant answer for itself, so the shop can stay open while a punchout
    /// channel serves only its negotiated assortment.
    pub async fn channels_visibility(&self, items: Vec<crate::models::ChannelVisibilityItem>, channel: Option<String>, channel_body: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/channels/visibility".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("items".to_string(), serde_json::to_value(&items)?);
        if let Some(value) = &channel {
            api_params.insert("channel".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &channel_body {
            api_params.insert("channel".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Discovery for the vocabulary routes: which enums this app publishes, not
    /// what is in them. An entry carries the name and the localised title and
    /// description a UI would put above a select, and stops there — the
    /// permitted values, their labels and their badge tones are the other route's
    /// answer. The split is deliberate. This index is a fixed, tiny answer a
    /// client can hold onto, while a vocabulary's contents are not fixed at all:
    /// `types` is backed by the tenant's own rows, so its values change whenever a
    /// merchant adds or retires one, and folding them in here would make every
    /// consumer re-fetch the whole set to learn a title. Names: statuses, types,
    /// unassigned-visibility. Fetch one with GET /channels/vocabularies/{name}; a
    /// client holding the qualified pair 'channels.<name>' builds that URL from
    /// the pair alone.
    pub async fn channels_vocabularies_list(&self) -> Result<crate::models::ChannelVocabularyIndex, Error> {
        let api_path = "/v1/channels/vocabularies".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One vocabulary with every permitted value in it, and a value here is more
    /// than the string the column stores: it arrives with the localised title and
    /// description a select puts in front of a person, and with a badge tone for
    /// rendering it as a status chip — `default_tone` is what a value carrying
    /// none falls back to, so there is always something to render. That is the
    /// whole reason this route exists rather than a client hardcoding the list.
    /// Two sources, one guarantee: what is served is what is in force, so no UI
    /// keeps a second copy. 'source' says which — 'schema' means the values are
    /// read out of the column's CHECK constraint (a value added to the constraint
    /// appears here even before anyone labels it, titled from its own key);
    /// 'table' means they are the tenant's own rows, which a merchant may add to,
    /// rename and retire without a release of this app. Values come back in author
    /// order, which is the order a select should offer. 'closed' says the set is
    /// exhaustive at this moment, so a value outside it is stale data rather than
    /// a missing label. Names: statuses, types, unassigned-visibility.
    pub async fn channels_vocabularies_get(&self, name: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/channels/vocabularies/{name}".replace("{name}", &name.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Nothing cascades from here, and that is a statement about the schema rather
    /// than a reassurance: this app declares no foreign key in either direction,
    /// so there is nothing to cascade TO. The channel ASSIGNMENTS other apps hold
    /// live in Baseline, keyed by the scope slug, and deleting the channel does
    /// not remove them. A slug that no longer names a channel simply stops
    /// resolving. The consequence is that the assignments OUTLIVE the row. Create
    /// a channel again under a code a deleted one used and it silently adopts
    /// every assignment ever made against that code — which is the opposite of
    /// the fresh channel the call looks like it produces. If that is not what you
    /// want, choose a new code. The other half is the default flag, which nothing
    /// here protects. There is no rule that a tenant keeps at least one channel
    /// and none reserving the one flagged `is_default` — both of which the
    /// channel TYPES do have — so deleting the default is permitted and leaves
    /// the tenant without one. From that moment every request that names no
    /// channel resolves to nothing: `GET /channels/context` answers resolved:false
    /// with reason no_default_channel, and `POST /channels/visibility` hides every
    /// row that carries assignments (no_channel_context) while rows carrying none
    /// still follow the tenant policy. Promote another channel first, or restore
    /// the seeded `shop` with POST /channels/defaults — which brings back
    /// `shop`, never the code you deleted.
    pub async fn channels_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/channels/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One row, by its uuid. The `code` is the handle everything else in the
    /// platform uses — it is the scope slug Baseline stores assignments against
    /// — and this route does not accept it: to go from a slug to the channel
    /// that owns it, use `GET /channels?code=…`, which answers the same row
    /// inside the list envelope. What this does NOT tell you is whether the
    /// request is in this channel. It returns an inactive channel as readily as an
    /// active one and applies no policy: which channel a caller is in, and what an
    /// unassigned row means there, is `GET /channels/context`. Answers are cached
    /// per tenant for 30 minutes and invalidated on any write to `channels`, so a
    /// read that follows someone else's write within that window can be stale by
    /// exactly one revision.
    pub async fn channels_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/channels/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A partial write: send the fields you are changing, keep the rest. An empty
    /// body is a 400 rather than a no-op, so a client that computed no diff hears
    /// about it. Two of these fields do more than they look like they do, and
    /// neither is guarded the way its counterpart on the channel TYPES is. Sending
    /// `code` is accepted — it is only checked for scope-slug shape — and
    /// nothing follows it: the assignments other apps made are held by Baseline
    /// against the OLD slug, there is no foreign key to cascade, so a rename
    /// silently detaches every one of them and the channel filters as if it had
    /// just been created. The types route refuses the same edit outright for the
    /// same reason; here it is permitted, so do it deliberately or not at all. And
    /// `is_default` is a two-way switch here. Setting it true demotes whoever held
    /// it, which is what you want; setting it FALSE on the only holder leaves the
    /// tenant with no default channel at all, and every request that names none
    /// then resolves to nothing — `GET /channels/context` answers resolved:false
    /// with reason no_default_channel. Promote another channel in the same breath.
    /// On the types route sending false does nothing, precisely because some row
    /// must hold that flag; channels have no such rule.
    pub async fn channels_update(&self, id: String, code: Option<String>, is_default: Option<bool>, labels: Option<serde_json::Value>, name: Option<String>, position: Option<i64>, status: Option<String>, xtype: Option<String>, unassigned_visibility: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/channels/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_default {
            api_params.insert("is_default".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &name {
            api_params.insert("name".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &status {
            api_params.insert("status".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &xtype {
            api_params.insert("type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &unassigned_visibility {
            api_params.insert("unassigned_visibility".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
