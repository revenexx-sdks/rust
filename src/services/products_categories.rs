use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// ProductsCategories service
pub struct ProductsCategories {
    client: Client,
}

impl ProductsCategories {
    pub fn new(client: Client) -> Self {
        ProductsCategories { client }
    }
    /// One node of the category tree. `parent_id` is the structure this app
    /// navigates — null is a root — while `path` is kept only for importers
    /// that carry one and nothing here reads or writes it. A category is
    /// hand-picked or RULE-DRIVEN: a non-null `rules` selector makes every
    /// matching product a `product_categories` row with source `rule`, alongside
    /// the hand-picked ones, and `rules_computed_at` says when that last
    /// completed.
    /// 
    /// Every column of `categories` is an exact-match query parameter, `order`
    /// sorts by one column, and `limit`/`offset` page through `page.total`. A
    /// query key that is NOT a column is dropped rather than refused, and the
    /// `filter` object echoes the ones that were understood — that echo is the
    /// only way to tell an unfiltered answer from an empty one. It reads rows
    /// exactly as they are stored: no join is resolved, no jsonb value is
    /// unpacked.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_categories_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, code: Option<String>, parent_id: Option<String>, path: Option<String>, position: Option<i64>, labels: Option<String>, values: Option<String>, rules: Option<String>, rule_match: Option<String>, rules_computed_at: Option<String>, created_at: Option<String>, updated_at: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/categories".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &offset {
            api_params.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order {
            api_params.insert("order".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &parent_id {
            api_params.insert("parent_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &path {
            api_params.insert("path".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &values {
            api_params.insert("values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &rules {
            api_params.insert("rules".to_string(), serde_json::to_value(value)?);
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
    /// Creates one category and answers 201 with the stored row, including the id
    /// and the timestamps the database filled in — a client never sends an id,
    /// it reads one back and uses it in the path of every later call.
    /// 
    /// One node of the category tree. `parent_id` is the structure this app
    /// navigates — null is a root — while `path` is kept only for importers
    /// that carry one and nothing here reads or writes it. A category is
    /// hand-picked or RULE-DRIVEN: a non-null `rules` selector makes every
    /// matching product a `product_categories` row with source `rule`, alongside
    /// the hand-picked ones, and `rules_computed_at` says when that last
    /// completed.
    /// 
    /// `code` is the only column the database refuses the row without; everything
    /// else has a default or is nullable. A second row with the same `code`
    /// answers 409.
    pub async fn products_categories_create(&self, code: String, labels: Option<serde_json::Value>, parent_id: Option<String>, path: Option<String>, position: Option<i64>, rule_match: Option<String>, rules: Option<serde_json::Value>, rules_computed_at: Option<String>, values: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/categories".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &parent_id {
            api_params.insert("parent_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &path {
            api_params.insert("path".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &rules_computed_at {
            api_params.insert("rules_computed_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &values {
            api_params.insert("values".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// What the nightly `recompute-category-rules` schedule calls, and the call to
    /// reach for after a bulk import has changed what the rules select. Same sync
    /// as the single-category recompute, applied to every category with non-null
    /// rules. The whole run shares ONE budget: a category the budget no longer
    /// reaches is reported as `skipped` and picked up by the next run, and a
    /// failing category is reported in its result entry instead of aborting the
    /// run.
    pub async fn products_categories_rules_recompute_all(&self, data: serde_json::Value) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/categories/rules/recompute-all".to_string();

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
    /// Dry-runs a rule: how many products it selects, plus a sample of up to ten,
    /// and it WRITES NOTHING. Evaluates the rule in the request body against the
    /// live catalog WITHOUT touching product_categories — this powers the
    /// cockpit's "matches N products" preview while an operator edits a rule.
    /// Soft-deleted products are excluded. Counting is delegated to the database,
    /// never enumerated: a rule that compiles to a single query is answered by one
    /// exact-count request whatever its match set. A rule that needs several
    /// queries (rule_match "any", or a repeated column such as a range) is
    /// combined in the app and stops at `cap` ids — check `capped` before
    /// showing `count` as a total.
    pub async fn products_categories_rules_preview(&self, category_id: String, conditions: Vec<crate::models::CategoryRuleCondition>, rule_match: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/categories/{category_id}/rules/preview".replace("{category_id}", &category_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("category_id".to_string(), serde_json::to_value(&category_id)?);
        api_params.insert("conditions".to_string(), serde_json::to_value(&conditions)?);
        if let Some(value) = &rule_match {
            api_params.insert("rule_match".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Syncs one category's rule-derived memberships to what its stored rule
    /// selects today. Evaluates categories.rules (NOT the request body), then
    /// inserts the newly matching products as source='rule' rows and deletes the
    /// rule rows that no longer match. Manual (source='manual') memberships are
    /// never inserted, deleted or shadowed. Stamps categories.rules_computed_at.
    /// 
    /// A large category does NOT finish in one call: the run stops when its
    /// wall-clock budget is spent and answers `done: false` with the `cursor` to
    /// send back, so drive it in a loop until `done` is true.
    pub async fn products_categories_rules_recompute(&self, category_id: String, cursor: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/categories/{category_id}/rules/recompute".replace("{category_id}", &category_id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("category_id".to_string(), serde_json::to_value(&category_id)?);
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
    /// Deletes one category by id. It is a hard delete — the row is gone, and
    /// the answer is a confirmation rather than a result to branch on.
    /// 
    /// It takes what hangs off it: product category memberships (`category_id`)
    /// are deleted with it. `categories.parent_id` is set to null instead, so the
    /// rows that pointed at it survive the delete rather than going with it.
    /// 
    /// An id no category of this tenant carries answers 404; there is no 409,
    /// because every foreign key pointing at this entity resolves itself on delete
    /// rather than blocking one.
    pub async fn products_categories_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/categories/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Reads one category by its id — the whole row, every column, as it is
    /// stored.
    /// 
    /// One node of the category tree. `parent_id` is the structure this app
    /// navigates — null is a root — while `path` is kept only for importers
    /// that carry one and nothing here reads or writes it. A category is
    /// hand-picked or RULE-DRIVEN: a non-null `rules` selector makes every
    /// matching product a `product_categories` row with source `rule`, alongside
    /// the hand-picked ones, and `rules_computed_at` says when that last
    /// completed.
    /// 
    /// An id no category of this tenant carries answers 404, and so does one
    /// belonging to another tenant: row-level security makes that row invisible
    /// rather than forbidden. A malformed id answers 400 before the route is
    /// reached.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_categories_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/categories/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Updates one category by id. A partial patch: the body names only the
    /// columns to change and every column it leaves out keeps its current value,
    /// so there is no read-modify-write and no way to blank a field by forgetting
    /// it.
    /// 
    /// One node of the category tree. `parent_id` is the structure this app
    /// navigates — null is a root — while `path` is kept only for importers
    /// that carry one and nothing here reads or writes it. A category is
    /// hand-picked or RULE-DRIVEN: a non-null `rules` selector makes every
    /// matching product a `product_categories` row with source `rule`, alongside
    /// the hand-picked ones, and `rules_computed_at` says when that last
    /// completed.
    /// 
    /// A body that names nothing writable is refused with 400 rather than answered
    /// as a no-op, an id nobody carries answers 404, and a value that collides on
    /// `code` answers 409.
    pub async fn products_categories_update(&self, id: String, code: Option<String>, labels: Option<serde_json::Value>, parent_id: Option<String>, path: Option<String>, position: Option<i64>, rule_match: Option<String>, rules: Option<serde_json::Value>, rules_computed_at: Option<String>, values: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/categories/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &parent_id {
            api_params.insert("parent_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &path {
            api_params.insert("path".to_string(), serde_json::to_value(value)?);
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
        if let Some(value) = &rules_computed_at {
            api_params.insert("rules_computed_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &values {
            api_params.insert("values".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One membership: this product is filed in this category. `source` says how
    /// it got there — `manual` is hand-picked, `rule` was materialized by a
    /// category rule — and the two never touch each other: a recompute only ever
    /// inserts and deletes `rule` rows, so a hand-picked membership survives every
    /// pass. `POST /products/{id}/categories` is the friendlier way to create one,
    /// because it takes the product from the path and answers with the category
    /// code and the SKU.
    /// 
    /// Every column of `product_categories` is an exact-match query parameter,
    /// `order` sorts by one column, and `limit`/`offset` page through
    /// `page.total`. A query key that is NOT a column is dropped rather than
    /// refused, and the `filter` object echoes the ones that were understood —
    /// that echo is the only way to tell an unfiltered answer from an empty one.
    /// It reads rows exactly as they are stored: no join is resolved, no jsonb
    /// value is unpacked.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_product_categories_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, product_id: Option<String>, category_id: Option<String>, position: Option<i64>, source: Option<String>, created_at: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/product_categories".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &limit {
            api_params.insert("limit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &offset {
            api_params.insert("offset".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &order {
            api_params.insert("order".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &product_id {
            api_params.insert("product_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &category_id {
            api_params.insert("category_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &source {
            api_params.insert("source".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &created_at {
            api_params.insert("created_at".to_string(), serde_json::to_value(value)?);
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
    /// Creates one product category membership and answers 201 with the stored
    /// row, including the id and the timestamps the database filled in — a
    /// client never sends an id, it reads one back and uses it in the path of
    /// every later call.
    /// 
    /// One membership: this product is filed in this category. `source` says how
    /// it got there — `manual` is hand-picked, `rule` was materialized by a
    /// category rule — and the two never touch each other: a recompute only ever
    /// inserts and deletes `rule` rows, so a hand-picked membership survives every
    /// pass. `POST /products/{id}/categories` is the friendlier way to create one,
    /// because it takes the product from the path and answers with the category
    /// code and the SKU.
    /// 
    /// `product_id` and `category_id` are the only columns the database refuses
    /// the row without; everything else has a default or is nullable. A second row
    /// with the same `product_id` and `category_id` answers 409.
    pub async fn products_product_categories_create(&self, category_id: String, product_id: String, position: Option<i64>, source: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/product_categories".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("category_id".to_string(), serde_json::to_value(&category_id)?);
        api_params.insert("product_id".to_string(), serde_json::to_value(&product_id)?);
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
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
    /// Deletes one product category membership by id. It is a hard delete — the
    /// row is gone, and the answer is a confirmation rather than a result to
    /// branch on.
    /// 
    /// Nothing in this schema references it, so nothing else changes.
    /// 
    /// An id no product category membership of this tenant carries answers 404;
    /// there is no 409, because every foreign key pointing at this entity resolves
    /// itself on delete rather than blocking one.
    pub async fn products_product_categories_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/product_categories/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Reads one product category membership by its id — the whole row, every
    /// column, as it is stored.
    /// 
    /// One membership: this product is filed in this category. `source` says how
    /// it got there — `manual` is hand-picked, `rule` was materialized by a
    /// category rule — and the two never touch each other: a recompute only ever
    /// inserts and deletes `rule` rows, so a hand-picked membership survives every
    /// pass. `POST /products/{id}/categories` is the friendlier way to create one,
    /// because it takes the product from the path and answers with the category
    /// code and the SKU.
    /// 
    /// An id no product category membership of this tenant carries answers 404,
    /// and so does one belonging to another tenant: row-level security makes that
    /// row invisible rather than forbidden. A malformed id answers 400 before the
    /// route is reached.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_product_categories_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/product_categories/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Updates one product category membership by id. A partial patch: the body
    /// names only the columns to change and every column it leaves out keeps its
    /// current value, so there is no read-modify-write and no way to blank a field
    /// by forgetting it.
    /// 
    /// One membership: this product is filed in this category. `source` says how
    /// it got there — `manual` is hand-picked, `rule` was materialized by a
    /// category rule — and the two never touch each other: a recompute only ever
    /// inserts and deletes `rule` rows, so a hand-picked membership survives every
    /// pass. `POST /products/{id}/categories` is the friendlier way to create one,
    /// because it takes the product from the path and answers with the category
    /// code and the SKU.
    /// 
    /// A body that names nothing writable is refused with 400 rather than answered
    /// as a no-op, an id nobody carries answers 404, and a value that collides on
    /// `product_id` and `category_id` answers 409.
    pub async fn products_product_categories_update(&self, id: String, category_id: Option<String>, position: Option<i64>, product_id: Option<String>, source: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/product_categories/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &category_id {
            api_params.insert("category_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &product_id {
            api_params.insert("product_id".to_string(), serde_json::to_value(value)?);
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
    /// Files one product into one category by hand, and the membership is always
    /// `source: 'manual'` — a rule recompute never deletes or shadows it.
    /// product_categories holds 28 758 rows and had no write surface that named
    /// the product it was filing. This takes the product from the route and the
    /// category from the body, which is what a bulk 'add the selected products to
    /// …' needs. The membership is always source='manual', so a rule recompute
    /// never deletes or shadows it.
    pub async fn products_categories_assign(&self, id: String, category_id: String, position: Option<i64>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/{id}/categories".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        api_params.insert("category_id".to_string(), serde_json::to_value(&category_id)?);
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
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
