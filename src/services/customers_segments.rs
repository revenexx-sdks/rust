use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// CustomersSegments service
pub struct CustomersSegments {
    client: Client,
}

impl CustomersSegments {
    pub fn new(client: Client) -> Self {
        CustomersSegments { client }
    }
    /// One organization inside one segment, plus the record of how it got there:
    /// `source: "manual"` for a company somebody put in, `source: "rule"` for one
    /// the rule engine matched. That distinction is what lets a recompute rewrite
    /// its own rows and leave every hand-picked one alone. The membership rows
    /// themselves — the answer to "which companies are in this segment"
    /// (`segment_id`) and to "which segments is this company in"
    /// (`organization_id`). Paged with `limit`/`offset`/`order`.
    pub async fn customers_segment_members_list(&self, id: Option<String>, segment_id: Option<String>, organization_id: Option<String>, source: Option<String>, created_at: Option<String>, limit: Option<i64>, offset: Option<i64>, order: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/segment_members".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &segment_id {
            api_params.insert("segment_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &organization_id {
            api_params.insert("organization_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &source {
            api_params.insert("source".to_string(), serde_json::to_value(value)?);
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
    /// One organization inside one segment, plus the record of how it got there:
    /// `source: "manual"` for a company somebody put in, `source: "rule"` for one
    /// the rule engine matched. That distinction is what lets a recompute rewrite
    /// its own rows and leave every hand-picked one alone. Adds a company to a
    /// segment BY HAND. The row is `source: "manual"`, which is what protects it:
    /// a rule recompute rewrites the rule-derived rows of that segment and never
    /// touches this one. A create cannot omit `segment_id` and `organization_id`;
    /// everything else is optional or defaulted by the database. Two rows of this
    /// tenant may not share the combination of `segment_id` + `organization_id`.
    pub async fn customers_segment_members_create(&self, organization_id: String, segment_id: String, source: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/segment_members".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("organization_id".to_string(), serde_json::to_value(&organization_id)?);
        api_params.insert("segment_id".to_string(), serde_json::to_value(&segment_id)?);
        if let Some(value) = &source {
            api_params.insert("source".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One organization inside one segment, plus the record of how it got there:
    /// `source: "manual"` for a company somebody put in, `source: "rule"` for one
    /// the rule engine matched. That distinction is what lets a recompute rewrite
    /// its own rows and leave every hand-picked one alone. Takes the company out
    /// of the segment. If the segment carries rules and the company still matches
    /// them, the next recompute puts it back; remove it from the rule, not from
    /// the list. Nothing else in this app points at it, so nothing else goes with
    /// it.
    pub async fn customers_segment_members_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/segment_members/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One organization inside one segment, plus the record of how it got there:
    /// `source: "manual"` for a company somebody put in, `source: "rule"` for one
    /// the rule engine matched. That distinction is what lets a recompute rewrite
    /// its own rows and leave every hand-picked one alone. One membership row by
    /// id, with the `source` that says how it came about.
    pub async fn customers_segment_members_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/segment_members/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One organization inside one segment, plus the record of how it got there:
    /// `source: "manual"` for a company somebody put in, `source: "rule"` for one
    /// the rule engine matched. That distinction is what lets a recompute rewrite
    /// its own rows and leave every hand-picked one alone. A partial update. In
    /// practice there is little to change — a membership is a pair of ids — so
    /// this exists for the `source` correction rather than as the normal path. Two
    /// rows of this tenant may not share the combination of `segment_id` +
    /// `organization_id`.
    pub async fn customers_segment_members_update(&self, id: String, organization_id: Option<String>, segment_id: Option<String>, source: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/segment_members/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &organization_id {
            api_params.insert("organization_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &segment_id {
            api_params.insert("segment_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &source {
            api_params.insert("source".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A segment is a named group of ORGANIZATIONS — never of people — built
    /// by hand, by rule, or both at once. It is what a price list, a campaign or a
    /// shipping option is pointed at when the answer is "these customers, not
    /// those". Every segment this tenant keeps, with its stored rules. Any column
    /// filters and the page is `limit`/`offset`/`order`. Which companies are
    /// actually IN one is `segment_members`, because the rule half is materialized
    /// rather than evaluated on read.
    pub async fn customers_segments_list(&self, id: Option<String>, code: Option<String>, position: Option<i64>, rule_match: Option<String>, rules_computed_at: Option<String>, created_at: Option<String>, updated_at: Option<String>, limit: Option<i64>, offset: Option<i64>, order: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/customers/segments".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &rule_match {
            api_params.insert("rule_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &rules_computed_at {
            api_params.insert("rules_computed_at".to_string(), serde_json::to_value(value)?);
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
    /// A segment is a named group of ORGANIZATIONS — never of people — built
    /// by hand, by rule, or both at once. It is what a price list, a campaign or a
    /// shipping option is pointed at when the answer is "these customers, not
    /// those". Creates the group. Rules are optional: leave them out for a
    /// hand-picked list, or store a rule document and let the recompute keep the
    /// membership up to date. The `code` is what other apps point at, so pick it
    /// deliberately. `code` is the only field a create cannot omit; everything
    /// else is optional or defaulted by the database. Two rows of this tenant may
    /// not share `code`.
    pub async fn customers_segments_create(&self, code: String, labels: Option<serde_json::Value>, position: Option<i64>, rule_match: Option<String>, rules: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/segments".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &rule_match {
            api_params.insert("rule_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &rules {
            api_params.insert("rules".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Same sync as the single-segment recompute, applied to every segment with
    /// non-null rules. A failing segment is reported in its result entry instead
    /// of aborting the run. The run shares one budget: a segment that does not fit
    /// reports done:false (or skipped:true) and keeps rules_computed_at null, so
    /// the next call resumes it from its own data. Repeat until the top-level done
    /// is true.
    pub async fn customers_segments_rules_recompute_all(&self, data: serde_json::Value) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/segments/rules/recompute-all".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("data".to_string(), serde_json::to_value(&data)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A segment is a named group of ORGANIZATIONS — never of people — built
    /// by hand, by rule, or both at once. It is what a price list, a campaign or a
    /// shipping option is pointed at when the answer is "these customers, not
    /// those". Removes the segment. Anything in another app that points at its
    /// `code` — a price list, a campaign — is left pointing at nothing,
    /// because no app may hold a foreign key into another (ADR-0055). Deleting one
    /// takes every `segment_members` row that points at it with it — the foreign
    /// keys decide, not this route.
    pub async fn customers_segments_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/segments/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A segment is a named group of ORGANIZATIONS — never of people — built
    /// by hand, by rule, or both at once. It is what a price list, a campaign or a
    /// shipping option is pointed at when the answer is "these customers, not
    /// those". One segment by id, including the rule document it carries. A
    /// segment with no rules is hand-picked and completely valid.
    pub async fn customers_segments_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/segments/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A segment is a named group of ORGANIZATIONS — never of people — built
    /// by hand, by rule, or both at once. It is what a price list, a campaign or a
    /// shipping option is pointed at when the answer is "these customers, not
    /// those". A partial update — send only what changes. Editing the rules does
    /// NOT re-evaluate them: that is `POST
    /// /customers/segments/{segment_id}/rules/recompute`, so a half-typed rule
    /// never silently empties a live segment. Two rows of this tenant may not
    /// share `code`.
    pub async fn customers_segments_update(&self, id: String, code: Option<String>, labels: Option<serde_json::Value>, position: Option<i64>, rule_match: Option<String>, rules: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/segments/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &rule_match {
            api_params.insert("rule_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &rules {
            api_params.insert("rules".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A dry run: it answers how many organizations the rule would select, with a
    /// handful of them by name, and writes nothing at all. Evaluates the rule
    /// document in the REQUEST BODY (not the stored segments.rules), so the
    /// cockpit can preview an unsaved rule. Costs a single count query for the
    /// common single-query rule; 'any' rules and rules repeating a column are
    /// combined in the app and capped at 5000 ids, in which case 'capped' is true
    /// and 'count' is a LOWER bound. Membership is never touched.
    pub async fn customers_segments_rules_preview(&self, segment_id: String, conditions: Vec<crate::models::SegmentRuleCondition>, rule_match: Option<String>, target: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/segments/{segment_id}/rules/preview".replace("{segment_id}", &segment_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("segment_id".to_string(), serde_json::to_value(&segment_id)?);
        api_params.insert("conditions".to_string(), serde_json::to_value(&conditions)?);
        if let Some(value) = &rule_match {
            api_params.insert("rule_match".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &target {
            api_params.insert("target".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Evaluates segments.rules (NOT the request body), then inserts the newly
    /// matching organizations as source='rule' rows and deletes the rule rows that
    /// no longer match. Manual (source='manual') memberships are never inserted,
    /// deleted or shadowed. Bounded by a wall-clock budget below the gateway's
    /// upstream timeout: when 'done' is false, POST again with the returned
    /// 'cursor' until it is true. added/removed/processed count THIS call only.
    /// Omitting 'cursor' resumes an unfinished pass and starts a fresh one after a
    /// completed pass; an explicit null always restarts.
    /// segments.rules_computed_at is stamped only when the pass completes.
    pub async fn customers_segments_rules_recompute(&self, segment_id: String, cursor: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/customers/segments/{segment_id}/rules/recompute".replace("{segment_id}", &segment_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("segment_id".to_string(), serde_json::to_value(&segment_id)?);
        if let Some(value) = &cursor {
            api_params.insert("cursor".to_string(), serde_json::to_value(value)?);
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
