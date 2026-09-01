use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// ProductsDataModel service
pub struct ProductsDataModel {
    client: Client,
}

impl ProductsDataModel {
    pub fn new(client: Client) -> Self {
        ProductsDataModel { client }
    }
    /// A class of media with one shared shape — packshots, datasheets, line
    /// drawings. The family decides which attributes an asset of it carries (alt
    /// text, copyright, an expiry date) and, through `naming_convention`, how a
    /// file of it is named — which is what lets an import bind a file to a
    /// product with no mapping table.
    /// 
    /// Every column of `asset_families` is an exact-match query parameter, `order`
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
    pub async fn products_asset_families_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, code: Option<String>, labels: Option<String>, naming_convention: Option<String>, created_at: Option<String>, updated_at: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/asset_families".to_string();

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
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &naming_convention {
            api_params.insert("naming_convention".to_string(), serde_json::to_value(value)?);
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
    /// Creates one asset family and answers 201 with the stored row, including the
    /// id and the timestamps the database filled in — a client never sends an
    /// id, it reads one back and uses it in the path of every later call.
    /// 
    /// A class of media with one shared shape — packshots, datasheets, line
    /// drawings. The family decides which attributes an asset of it carries (alt
    /// text, copyright, an expiry date) and, through `naming_convention`, how a
    /// file of it is named — which is what lets an import bind a file to a
    /// product with no mapping table.
    /// 
    /// `code` is the only column the database refuses the row without; everything
    /// else has a default or is nullable. A second row with the same `code`
    /// answers 409.
    pub async fn products_asset_families_create(&self, code: String, labels: Option<serde_json::Value>, naming_convention: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/asset_families".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &naming_convention {
            api_params.insert("naming_convention".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Deletes one asset family by id. It is a hard delete — the row is gone,
    /// and the answer is a confirmation rather than a result to branch on.
    /// 
    /// It takes what hangs off it: assets (`asset_family_id`) are deleted with it.
    /// 
    /// An id no asset family of this tenant carries answers 404; there is no 409,
    /// because every foreign key pointing at this entity resolves itself on delete
    /// rather than blocking one.
    pub async fn products_asset_families_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/asset_families/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Reads one asset family by its id — the whole row, every column, as it is
    /// stored.
    /// 
    /// A class of media with one shared shape — packshots, datasheets, line
    /// drawings. The family decides which attributes an asset of it carries (alt
    /// text, copyright, an expiry date) and, through `naming_convention`, how a
    /// file of it is named — which is what lets an import bind a file to a
    /// product with no mapping table.
    /// 
    /// An id no asset family of this tenant carries answers 404, and so does one
    /// belonging to another tenant: row-level security makes that row invisible
    /// rather than forbidden. A malformed id answers 400 before the route is
    /// reached.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_asset_families_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/asset_families/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Updates one asset family by id. A partial patch: the body names only the
    /// columns to change and every column it leaves out keeps its current value,
    /// so there is no read-modify-write and no way to blank a field by forgetting
    /// it.
    /// 
    /// A class of media with one shared shape — packshots, datasheets, line
    /// drawings. The family decides which attributes an asset of it carries (alt
    /// text, copyright, an expiry date) and, through `naming_convention`, how a
    /// file of it is named — which is what lets an import bind a file to a
    /// product with no mapping table.
    /// 
    /// A body that names nothing writable is refused with 400 rather than answered
    /// as a no-op, an id nobody carries answers 404, and a value that collides on
    /// `code` answers 409.
    pub async fn products_asset_families_update(&self, id: String, code: Option<String>, labels: Option<serde_json::Value>, naming_convention: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/asset_families/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &naming_convention {
            api_params.insert("naming_convention".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The KIND of relation two products can have — cross-sell, accessory, spare
    /// part, bill of materials. `is_two_way` declares the relation symmetric and
    /// `is_quantified` declares that it carries a quantity; both are declarations
    /// a client READS rather than behaviour this app performs — it stores one
    /// row per direction and never creates the mirror for you.
    /// 
    /// Every column of `association_types` is an exact-match query parameter,
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
    pub async fn products_association_types_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, code: Option<String>, is_two_way: Option<bool>, is_quantified: Option<bool>, labels: Option<String>, created_at: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/association_types".to_string();

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
        if let Some(value) = &is_two_way {
            api_params.insert("is_two_way".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_quantified {
            api_params.insert("is_quantified".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
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
    /// Creates one association type and answers 201 with the stored row, including
    /// the id and the timestamps the database filled in — a client never sends
    /// an id, it reads one back and uses it in the path of every later call.
    /// 
    /// The KIND of relation two products can have — cross-sell, accessory, spare
    /// part, bill of materials. `is_two_way` declares the relation symmetric and
    /// `is_quantified` declares that it carries a quantity; both are declarations
    /// a client READS rather than behaviour this app performs — it stores one
    /// row per direction and never creates the mirror for you.
    /// 
    /// `code` is the only column the database refuses the row without; everything
    /// else has a default or is nullable. A second row with the same `code`
    /// answers 409.
    pub async fn products_association_types_create(&self, code: String, is_quantified: Option<bool>, is_two_way: Option<bool>, labels: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/association_types".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &is_quantified {
            api_params.insert("is_quantified".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_two_way {
            api_params.insert("is_two_way".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Deletes one association type by id. It is a hard delete — the row is
    /// gone, and the answer is a confirmation rather than a result to branch on.
    /// 
    /// It takes what hangs off it: product associations (`association_type_id`)
    /// are deleted with it.
    /// 
    /// An id no association type of this tenant carries answers 404; there is no
    /// 409, because every foreign key pointing at this entity resolves itself on
    /// delete rather than blocking one.
    pub async fn products_association_types_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/association_types/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Reads one association type by its id — the whole row, every column, as it
    /// is stored.
    /// 
    /// The KIND of relation two products can have — cross-sell, accessory, spare
    /// part, bill of materials. `is_two_way` declares the relation symmetric and
    /// `is_quantified` declares that it carries a quantity; both are declarations
    /// a client READS rather than behaviour this app performs — it stores one
    /// row per direction and never creates the mirror for you.
    /// 
    /// An id no association type of this tenant carries answers 404, and so does
    /// one belonging to another tenant: row-level security makes that row
    /// invisible rather than forbidden. A malformed id answers 400 before the
    /// route is reached.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_association_types_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/association_types/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Updates one association type by id. A partial patch: the body names only
    /// the columns to change and every column it leaves out keeps its current
    /// value, so there is no read-modify-write and no way to blank a field by
    /// forgetting it.
    /// 
    /// The KIND of relation two products can have — cross-sell, accessory, spare
    /// part, bill of materials. `is_two_way` declares the relation symmetric and
    /// `is_quantified` declares that it carries a quantity; both are declarations
    /// a client READS rather than behaviour this app performs — it stores one
    /// row per direction and never creates the mirror for you.
    /// 
    /// A body that names nothing writable is refused with 400 rather than answered
    /// as a no-op, an id nobody carries answers 404, and a value that collides on
    /// `code` answers 409.
    pub async fn products_association_types_update(&self, id: String, code: Option<String>, is_quantified: Option<bool>, is_two_way: Option<bool>, labels: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/association_types/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_quantified {
            api_params.insert("is_quantified".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_two_way {
            api_params.insert("is_two_way".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Which fields does this family have — one ready-to-render list, not six
    /// joined tables. The catalog's SHAPE is tenant data: a product's properties
    /// are rows in `attributes`, grouped by `attribute_groups`, selected per
    /// family by `family_attributes`, with their permitted values in
    /// `attribute_options` and their variant axes in `family_variants`. Reading
    /// that shape used to mean five reads, a join, and a private `attributes.type`
    /// → input mapping in every client — and that mapping is the part that
    /// must live here, because the type list carries no CHECK by design and an
    /// integrator extends it. Answers one field list instead, ordered by group
    /// then by the family's own ordering. Without a family it answers every
    /// attribute declared for `entity_type`/`entity_ref` — the shape of a
    /// reference entity's records or an asset family, which have attributes but no
    /// family. Writes nothing.
    pub async fn products_attribute_schema(&self, family_id: Option<String>, family_code: Option<String>, entity_type: Option<String>, entity_ref: Option<String>, locale: Option<String>, channel: Option<String>, kind: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/attribute-schema".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &family_id {
            api_params.insert("family_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &family_code {
            api_params.insert("family_code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entity_type {
            api_params.insert("entity_type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entity_ref {
            api_params.insert("entity_ref".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &locale {
            api_params.insert("locale".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &channel {
            api_params.insert("channel".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &kind {
            api_params.insert("kind".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// An attribute group is a SECTION of a product form — "Technical
    /// attributes", "Logistics" — and the thing every attribute is filed under.
    /// It carries a `position`, which is the order the sections appear in, and
    /// per-language `labels`, which is what an operator reads; the `code` is what
    /// an attribute joins on and is never shown. `GET /products/attribute-schema`
    /// already resolves a group's heading onto every field it returns, so these
    /// routes are for MANAGING the sections, not for rendering a form.
    /// 
    /// Every column of `attribute_groups` is an exact-match query parameter,
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
    pub async fn products_attribute_groups_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, code: Option<String>, position: Option<i64>, labels: Option<String>, created_at: Option<String>, updated_at: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/attribute_groups".to_string();

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
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
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
    /// Creates one attribute group and answers 201 with the stored row, including
    /// the id and the timestamps the database filled in — a client never sends
    /// an id, it reads one back and uses it in the path of every later call.
    /// 
    /// An attribute group is a SECTION of a product form — "Technical
    /// attributes", "Logistics" — and the thing every attribute is filed under.
    /// It carries a `position`, which is the order the sections appear in, and
    /// per-language `labels`, which is what an operator reads; the `code` is what
    /// an attribute joins on and is never shown. `GET /products/attribute-schema`
    /// already resolves a group's heading onto every field it returns, so these
    /// routes are for MANAGING the sections, not for rendering a form.
    /// 
    /// `code` is the only column the database refuses the row without; everything
    /// else has a default or is nullable. A second row with the same `code`
    /// answers 409.
    pub async fn products_attribute_groups_create(&self, code: String, labels: Option<serde_json::Value>, position: Option<i64>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/attribute_groups".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
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
    /// Deletes one attribute group by id. It is a hard delete — the row is gone,
    /// and the answer is a confirmation rather than a result to branch on.
    /// 
    /// `attributes.group_id` is set to null instead, so the rows that pointed at
    /// it survive the delete rather than going with it.
    /// 
    /// An id no attribute group of this tenant carries answers 404; there is no
    /// 409, because every foreign key pointing at this entity resolves itself on
    /// delete rather than blocking one.
    pub async fn products_attribute_groups_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/attribute_groups/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Reads one attribute group by its id — the whole row, every column, as it
    /// is stored.
    /// 
    /// An attribute group is a SECTION of a product form — "Technical
    /// attributes", "Logistics" — and the thing every attribute is filed under.
    /// It carries a `position`, which is the order the sections appear in, and
    /// per-language `labels`, which is what an operator reads; the `code` is what
    /// an attribute joins on and is never shown. `GET /products/attribute-schema`
    /// already resolves a group's heading onto every field it returns, so these
    /// routes are for MANAGING the sections, not for rendering a form.
    /// 
    /// An id no attribute group of this tenant carries answers 404, and so does
    /// one belonging to another tenant: row-level security makes that row
    /// invisible rather than forbidden. A malformed id answers 400 before the
    /// route is reached.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_attribute_groups_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/attribute_groups/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Updates one attribute group by id. A partial patch: the body names only the
    /// columns to change and every column it leaves out keeps its current value,
    /// so there is no read-modify-write and no way to blank a field by forgetting
    /// it.
    /// 
    /// An attribute group is a SECTION of a product form — "Technical
    /// attributes", "Logistics" — and the thing every attribute is filed under.
    /// It carries a `position`, which is the order the sections appear in, and
    /// per-language `labels`, which is what an operator reads; the `code` is what
    /// an attribute joins on and is never shown. `GET /products/attribute-schema`
    /// already resolves a group's heading onto every field it returns, so these
    /// routes are for MANAGING the sections, not for rendering a form.
    /// 
    /// A body that names nothing writable is refused with 400 rather than answered
    /// as a no-op, an id nobody carries answers 404, and a value that collides on
    /// `code` answers 409.
    pub async fn products_attribute_groups_update(&self, id: String, code: Option<String>, labels: Option<serde_json::Value>, position: Option<i64>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/attribute_groups/{id}".replace("{id}", &id.to_string());

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

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The permitted values of one select or multi-select attribute. A record
    /// stores the option's CODE and never its label, so renaming an option in
    /// every language leaves every product that picked it untouched, and
    /// `position` is the order it appears in the dropdown. `GET
    /// /products/attribute-schema` republishes these as a field's `options`,
    /// already resolved for a locale.
    /// 
    /// Every column of `attribute_options` is an exact-match query parameter,
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
    pub async fn products_attribute_options_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, attribute_id: Option<String>, code: Option<String>, position: Option<i64>, swatch: Option<String>, labels: Option<String>, created_at: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/attribute_options".to_string();

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
        if let Some(value) = &attribute_id {
            api_params.insert("attribute_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &swatch {
            api_params.insert("swatch".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
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
    /// Creates one attribute option and answers 201 with the stored row, including
    /// the id and the timestamps the database filled in — a client never sends
    /// an id, it reads one back and uses it in the path of every later call.
    /// 
    /// The permitted values of one select or multi-select attribute. A record
    /// stores the option's CODE and never its label, so renaming an option in
    /// every language leaves every product that picked it untouched, and
    /// `position` is the order it appears in the dropdown. `GET
    /// /products/attribute-schema` republishes these as a field's `options`,
    /// already resolved for a locale.
    /// 
    /// `attribute_id` and `code` are the only columns the database refuses the row
    /// without; everything else has a default or is nullable. A second row with
    /// the same `attribute_id` and `code` answers 409.
    pub async fn products_attribute_options_create(&self, attribute_id: String, code: String, labels: Option<serde_json::Value>, position: Option<i64>, swatch: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/attribute_options".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("attribute_id".to_string(), serde_json::to_value(&attribute_id)?);
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &swatch {
            api_params.insert("swatch".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Deletes one attribute option by id. It is a hard delete — the row is
    /// gone, and the answer is a confirmation rather than a result to branch on.
    /// 
    /// Nothing in this schema references it, so nothing else changes.
    /// 
    /// An id no attribute option of this tenant carries answers 404; there is no
    /// 409, because every foreign key pointing at this entity resolves itself on
    /// delete rather than blocking one.
    pub async fn products_attribute_options_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/attribute_options/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Reads one attribute option by its id — the whole row, every column, as it
    /// is stored.
    /// 
    /// The permitted values of one select or multi-select attribute. A record
    /// stores the option's CODE and never its label, so renaming an option in
    /// every language leaves every product that picked it untouched, and
    /// `position` is the order it appears in the dropdown. `GET
    /// /products/attribute-schema` republishes these as a field's `options`,
    /// already resolved for a locale.
    /// 
    /// An id no attribute option of this tenant carries answers 404, and so does
    /// one belonging to another tenant: row-level security makes that row
    /// invisible rather than forbidden. A malformed id answers 400 before the
    /// route is reached.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_attribute_options_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/attribute_options/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Updates one attribute option by id. A partial patch: the body names only
    /// the columns to change and every column it leaves out keeps its current
    /// value, so there is no read-modify-write and no way to blank a field by
    /// forgetting it.
    /// 
    /// The permitted values of one select or multi-select attribute. A record
    /// stores the option's CODE and never its label, so renaming an option in
    /// every language leaves every product that picked it untouched, and
    /// `position` is the order it appears in the dropdown. `GET
    /// /products/attribute-schema` republishes these as a field's `options`,
    /// already resolved for a locale.
    /// 
    /// A body that names nothing writable is refused with 400 rather than answered
    /// as a no-op, an id nobody carries answers 404, and a value that collides on
    /// `attribute_id` and `code` answers 409.
    pub async fn products_attribute_options_update(&self, id: String, attribute_id: Option<String>, code: Option<String>, labels: Option<serde_json::Value>, position: Option<i64>, swatch: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/attribute_options/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &attribute_id {
            api_params.insert("attribute_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &swatch {
            api_params.insert("swatch".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// An attribute is one property a record can carry, and in an attribute-driven
    /// PIM it is a ROW rather than a column: giving the catalog a "net weight" is
    /// a create here, not a migration. Its own flags decide everything downstream
    /// — `localizable` and `scopable` pick which of the four `attribute_values`
    /// buckets its values are written to, `type` picks the editor that renders it,
    /// `usable_in_grid` and `is_filterable` are what the product grid reads.
    /// `entity_type`/`entity_ref` say which kind of record carries it: a product,
    /// one reference entity's records, one asset family, or a category.
    /// 
    /// Every column of `attributes` is an exact-match query parameter, `order`
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
    pub async fn products_attributes_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, code: Option<String>, entity_type: Option<String>, entity_ref: Option<String>, xtype: Option<String>, group_id: Option<String>, localizable: Option<bool>, scopable: Option<bool>, is_unique: Option<bool>, is_filterable: Option<bool>, usable_in_grid: Option<bool>, validation: Option<String>, config: Option<String>, labels: Option<String>, position: Option<i64>, created_at: Option<String>, updated_at: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/attributes".to_string();

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
        if let Some(value) = &entity_type {
            api_params.insert("entity_type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entity_ref {
            api_params.insert("entity_ref".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &xtype {
            api_params.insert("type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &group_id {
            api_params.insert("group_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &localizable {
            api_params.insert("localizable".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &scopable {
            api_params.insert("scopable".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_unique {
            api_params.insert("is_unique".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_filterable {
            api_params.insert("is_filterable".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &usable_in_grid {
            api_params.insert("usable_in_grid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &validation {
            api_params.insert("validation".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &config {
            api_params.insert("config".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
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
    /// Creates one attribute and answers 201 with the stored row, including the id
    /// and the timestamps the database filled in — a client never sends an id,
    /// it reads one back and uses it in the path of every later call.
    /// 
    /// An attribute is one property a record can carry, and in an attribute-driven
    /// PIM it is a ROW rather than a column: giving the catalog a "net weight" is
    /// a create here, not a migration. Its own flags decide everything downstream
    /// — `localizable` and `scopable` pick which of the four `attribute_values`
    /// buckets its values are written to, `type` picks the editor that renders it,
    /// `usable_in_grid` and `is_filterable` are what the product grid reads.
    /// `entity_type`/`entity_ref` say which kind of record carries it: a product,
    /// one reference entity's records, one asset family, or a category.
    /// 
    /// `code` and `type` are the only columns the database refuses the row
    /// without; everything else has a default or is nullable. A second row with
    /// the same `entity_type`, `entity_ref`, `code` answers 409.
    pub async fn products_attributes_create(&self, code: String, xtype: String, config: Option<serde_json::Value>, entity_ref: Option<String>, entity_type: Option<String>, group_id: Option<String>, is_filterable: Option<bool>, is_unique: Option<bool>, labels: Option<serde_json::Value>, localizable: Option<bool>, position: Option<i64>, scopable: Option<bool>, usable_in_grid: Option<bool>, validation: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/attributes".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("type".to_string(), serde_json::to_value(&xtype)?);
        if let Some(value) = &config {
            api_params.insert("config".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entity_ref {
            api_params.insert("entity_ref".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entity_type {
            api_params.insert("entity_type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &group_id {
            api_params.insert("group_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_filterable {
            api_params.insert("is_filterable".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_unique {
            api_params.insert("is_unique".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &localizable {
            api_params.insert("localizable".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &scopable {
            api_params.insert("scopable".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &usable_in_grid {
            api_params.insert("usable_in_grid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &validation {
            api_params.insert("validation".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Deletes one attribute by id. It is a hard delete — the row is gone, and
    /// the answer is a confirmation rather than a result to branch on.
    /// 
    /// It takes what hangs off it: attribute options (`attribute_id`), family
    /// attributes (`attribute_id`) are deleted with it.
    /// 
    /// An id no attribute of this tenant carries answers 404; there is no 409,
    /// because every foreign key pointing at this entity resolves itself on delete
    /// rather than blocking one.
    pub async fn products_attributes_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/attributes/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Reads one attribute by its id — the whole row, every column, as it is
    /// stored.
    /// 
    /// An attribute is one property a record can carry, and in an attribute-driven
    /// PIM it is a ROW rather than a column: giving the catalog a "net weight" is
    /// a create here, not a migration. Its own flags decide everything downstream
    /// — `localizable` and `scopable` pick which of the four `attribute_values`
    /// buckets its values are written to, `type` picks the editor that renders it,
    /// `usable_in_grid` and `is_filterable` are what the product grid reads.
    /// `entity_type`/`entity_ref` say which kind of record carries it: a product,
    /// one reference entity's records, one asset family, or a category.
    /// 
    /// An id no attribute of this tenant carries answers 404, and so does one
    /// belonging to another tenant: row-level security makes that row invisible
    /// rather than forbidden. A malformed id answers 400 before the route is
    /// reached.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_attributes_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/attributes/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Updates one attribute by id. A partial patch: the body names only the
    /// columns to change and every column it leaves out keeps its current value,
    /// so there is no read-modify-write and no way to blank a field by forgetting
    /// it.
    /// 
    /// An attribute is one property a record can carry, and in an attribute-driven
    /// PIM it is a ROW rather than a column: giving the catalog a "net weight" is
    /// a create here, not a migration. Its own flags decide everything downstream
    /// — `localizable` and `scopable` pick which of the four `attribute_values`
    /// buckets its values are written to, `type` picks the editor that renders it,
    /// `usable_in_grid` and `is_filterable` are what the product grid reads.
    /// `entity_type`/`entity_ref` say which kind of record carries it: a product,
    /// one reference entity's records, one asset family, or a category.
    /// 
    /// A body that names nothing writable is refused with 400 rather than answered
    /// as a no-op, an id nobody carries answers 404, and a value that collides on
    /// `entity_type`, `entity_ref`, `code` answers 409.
    pub async fn products_attributes_update(&self, id: String, code: Option<String>, config: Option<serde_json::Value>, entity_ref: Option<String>, entity_type: Option<String>, group_id: Option<String>, is_filterable: Option<bool>, is_unique: Option<bool>, labels: Option<serde_json::Value>, localizable: Option<bool>, position: Option<i64>, scopable: Option<bool>, xtype: Option<String>, usable_in_grid: Option<bool>, validation: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/attributes/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &config {
            api_params.insert("config".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entity_ref {
            api_params.insert("entity_ref".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &entity_type {
            api_params.insert("entity_type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &group_id {
            api_params.insert("group_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_filterable {
            api_params.insert("is_filterable".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_unique {
            api_params.insert("is_unique".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &localizable {
            api_params.insert("localizable".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &scopable {
            api_params.insert("scopable".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &xtype {
            api_params.insert("type".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &usable_in_grid {
            api_params.insert("usable_in_grid".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &validation {
            api_params.insert("validation".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A family decides WHICH attributes a product has — the set is
    /// `family_attributes`, and every family-driven surface follows from it. It
    /// also names which attribute carries the display name (`label_attribute`) and
    /// which carries the main image. A product with no family has no required
    /// attributes at all, so its completeness cannot be measured and its name
    /// never resolves past the SKU; `POST /products/{id}/family` is the call that
    /// ends that state.
    /// 
    /// Every column of `families` is an exact-match query parameter, `order` sorts
    /// by one column, and `limit`/`offset` page through `page.total`. A query key
    /// that is NOT a column is dropped rather than refused, and the `filter`
    /// object echoes the ones that were understood — that echo is the only way
    /// to tell an unfiltered answer from an empty one. It reads rows exactly as
    /// they are stored: no join is resolved, no jsonb value is unpacked.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_families_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, code: Option<String>, label_attribute: Option<String>, image_attribute: Option<String>, labels: Option<String>, created_at: Option<String>, updated_at: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/families".to_string();

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
        if let Some(value) = &label_attribute {
            api_params.insert("label_attribute".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &image_attribute {
            api_params.insert("image_attribute".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
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
    /// Creates one family and answers 201 with the stored row, including the id
    /// and the timestamps the database filled in — a client never sends an id,
    /// it reads one back and uses it in the path of every later call.
    /// 
    /// A family decides WHICH attributes a product has — the set is
    /// `family_attributes`, and every family-driven surface follows from it. It
    /// also names which attribute carries the display name (`label_attribute`) and
    /// which carries the main image. A product with no family has no required
    /// attributes at all, so its completeness cannot be measured and its name
    /// never resolves past the SKU; `POST /products/{id}/family` is the call that
    /// ends that state.
    /// 
    /// `code` is the only column the database refuses the row without; everything
    /// else has a default or is nullable. A second row with the same `code`
    /// answers 409.
    pub async fn products_families_create(&self, code: String, image_attribute: Option<String>, label_attribute: Option<String>, labels: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/families".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &image_attribute {
            api_params.insert("image_attribute".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &label_attribute {
            api_params.insert("label_attribute".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Deletes one family by id. It is a hard delete — the row is gone, and the
    /// answer is a confirmation rather than a result to branch on.
    /// 
    /// It takes what hangs off it: family attributes (`family_id`), family
    /// variants (`family_id`) are deleted with it. `products.family_id` is set to
    /// null instead, so the rows that pointed at it survive the delete rather than
    /// going with it.
    /// 
    /// An id no family of this tenant carries answers 404; there is no 409,
    /// because every foreign key pointing at this entity resolves itself on delete
    /// rather than blocking one.
    pub async fn products_families_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/families/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Reads one family by its id — the whole row, every column, as it is
    /// stored.
    /// 
    /// A family decides WHICH attributes a product has — the set is
    /// `family_attributes`, and every family-driven surface follows from it. It
    /// also names which attribute carries the display name (`label_attribute`) and
    /// which carries the main image. A product with no family has no required
    /// attributes at all, so its completeness cannot be measured and its name
    /// never resolves past the SKU; `POST /products/{id}/family` is the call that
    /// ends that state.
    /// 
    /// An id no family of this tenant carries answers 404, and so does one
    /// belonging to another tenant: row-level security makes that row invisible
    /// rather than forbidden. A malformed id answers 400 before the route is
    /// reached.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_families_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/families/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Updates one family by id. A partial patch: the body names only the columns
    /// to change and every column it leaves out keeps its current value, so there
    /// is no read-modify-write and no way to blank a field by forgetting it.
    /// 
    /// A family decides WHICH attributes a product has — the set is
    /// `family_attributes`, and every family-driven surface follows from it. It
    /// also names which attribute carries the display name (`label_attribute`) and
    /// which carries the main image. A product with no family has no required
    /// attributes at all, so its completeness cannot be measured and its name
    /// never resolves past the SKU; `POST /products/{id}/family` is the call that
    /// ends that state.
    /// 
    /// A body that names nothing writable is refused with 400 rather than answered
    /// as a no-op, an id nobody carries answers 404, and a value that collides on
    /// `code` answers 409.
    pub async fn products_families_update(&self, id: String, code: Option<String>, image_attribute: Option<String>, label_attribute: Option<String>, labels: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/families/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &image_attribute {
            api_params.insert("image_attribute".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &label_attribute {
            api_params.insert("label_attribute".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One link between a family and an attribute — the row that puts an
    /// attribute INTO a family's form. It carries the family's own ordering of
    /// that attribute, which overrides the attribute's default position, and
    /// `is_required`, which is the flag `POST /products/{id}/completeness`
    /// measures and nothing else reads. `required_channels` narrows "required" to
    /// named channels; null or empty means required EVERYWHERE, not nowhere.
    /// 
    /// Every column of `family_attributes` is an exact-match query parameter,
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
    pub async fn products_family_attributes_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, family_id: Option<String>, attribute_id: Option<String>, position: Option<i64>, is_required: Option<bool>, required_channels: Option<String>, created_at: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/family_attributes".to_string();

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
        if let Some(value) = &family_id {
            api_params.insert("family_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &attribute_id {
            api_params.insert("attribute_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_required {
            api_params.insert("is_required".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &required_channels {
            api_params.insert("required_channels".to_string(), serde_json::to_value(value)?);
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
    /// Creates one family attribute and answers 201 with the stored row, including
    /// the id and the timestamps the database filled in — a client never sends
    /// an id, it reads one back and uses it in the path of every later call.
    /// 
    /// One link between a family and an attribute — the row that puts an
    /// attribute INTO a family's form. It carries the family's own ordering of
    /// that attribute, which overrides the attribute's default position, and
    /// `is_required`, which is the flag `POST /products/{id}/completeness`
    /// measures and nothing else reads. `required_channels` narrows "required" to
    /// named channels; null or empty means required EVERYWHERE, not nowhere.
    /// 
    /// `family_id` and `attribute_id` are the only columns the database refuses
    /// the row without; everything else has a default or is nullable. A second row
    /// with the same `family_id` and `attribute_id` answers 409.
    pub async fn products_family_attributes_create(&self, attribute_id: String, family_id: String, is_required: Option<bool>, position: Option<i64>, required_channels: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/family_attributes".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("attribute_id".to_string(), serde_json::to_value(&attribute_id)?);
        api_params.insert("family_id".to_string(), serde_json::to_value(&family_id)?);
        if let Some(value) = &is_required {
            api_params.insert("is_required".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &required_channels {
            api_params.insert("required_channels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Deletes one family attribute by id. It is a hard delete — the row is
    /// gone, and the answer is a confirmation rather than a result to branch on.
    /// 
    /// Nothing in this schema references it, so nothing else changes.
    /// 
    /// An id no family attribute of this tenant carries answers 404; there is no
    /// 409, because every foreign key pointing at this entity resolves itself on
    /// delete rather than blocking one.
    pub async fn products_family_attributes_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/family_attributes/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Reads one family attribute by its id — the whole row, every column, as it
    /// is stored.
    /// 
    /// One link between a family and an attribute — the row that puts an
    /// attribute INTO a family's form. It carries the family's own ordering of
    /// that attribute, which overrides the attribute's default position, and
    /// `is_required`, which is the flag `POST /products/{id}/completeness`
    /// measures and nothing else reads. `required_channels` narrows "required" to
    /// named channels; null or empty means required EVERYWHERE, not nowhere.
    /// 
    /// An id no family attribute of this tenant carries answers 404, and so does
    /// one belonging to another tenant: row-level security makes that row
    /// invisible rather than forbidden. A malformed id answers 400 before the
    /// route is reached.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_family_attributes_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/family_attributes/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Updates one family attribute by id. A partial patch: the body names only
    /// the columns to change and every column it leaves out keeps its current
    /// value, so there is no read-modify-write and no way to blank a field by
    /// forgetting it.
    /// 
    /// One link between a family and an attribute — the row that puts an
    /// attribute INTO a family's form. It carries the family's own ordering of
    /// that attribute, which overrides the attribute's default position, and
    /// `is_required`, which is the flag `POST /products/{id}/completeness`
    /// measures and nothing else reads. `required_channels` narrows "required" to
    /// named channels; null or empty means required EVERYWHERE, not nowhere.
    /// 
    /// A body that names nothing writable is refused with 400 rather than answered
    /// as a no-op, an id nobody carries answers 404, and a value that collides on
    /// `family_id` and `attribute_id` answers 409.
    pub async fn products_family_attributes_update(&self, id: String, attribute_id: Option<String>, family_id: Option<String>, is_required: Option<bool>, position: Option<i64>, required_channels: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/family_attributes/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &attribute_id {
            api_params.insert("attribute_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &family_id {
            api_params.insert("family_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &is_required {
            api_params.insert("is_required".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &required_channels {
            api_params.insert("required_channels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A variant structure of a family: the attribute axes a product model splits
    /// its variants on — colour, then size. A product follows one through
    /// `family_variant_id`, and an attribute named as an axis becomes read-only on
    /// the model and is set on each variant instead, which is what `GET
    /// /products/attribute-schema` reports as `readonly_reason`. Two axis shapes
    /// are in the wild and both are read: a bare list of codes, or one entry per
    /// level.
    /// 
    /// Every column of `family_variants` is an exact-match query parameter,
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
    pub async fn products_family_variants_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, family_id: Option<String>, code: Option<String>, labels: Option<String>, axes: Option<String>, created_at: Option<String>, updated_at: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/family_variants".to_string();

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
        if let Some(value) = &family_id {
            api_params.insert("family_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &axes {
            api_params.insert("axes".to_string(), serde_json::to_value(value)?);
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
    /// Creates one family variant and answers 201 with the stored row, including
    /// the id and the timestamps the database filled in — a client never sends
    /// an id, it reads one back and uses it in the path of every later call.
    /// 
    /// A variant structure of a family: the attribute axes a product model splits
    /// its variants on — colour, then size. A product follows one through
    /// `family_variant_id`, and an attribute named as an axis becomes read-only on
    /// the model and is set on each variant instead, which is what `GET
    /// /products/attribute-schema` reports as `readonly_reason`. Two axis shapes
    /// are in the wild and both are read: a bare list of codes, or one entry per
    /// level.
    /// 
    /// `family_id` and `code` are the only columns the database refuses the row
    /// without; everything else has a default or is nullable. A second row with
    /// the same `code` answers 409.
    pub async fn products_family_variants_create(&self, code: String, family_id: String, axes: Option<serde_json::Value>, labels: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/family_variants".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("family_id".to_string(), serde_json::to_value(&family_id)?);
        if let Some(value) = &axes {
            api_params.insert("axes".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Deletes one family variant by id. It is a hard delete — the row is gone,
    /// and the answer is a confirmation rather than a result to branch on.
    /// 
    /// `products.family_variant_id` is set to null instead, so the rows that
    /// pointed at it survive the delete rather than going with it.
    /// 
    /// An id no family variant of this tenant carries answers 404; there is no
    /// 409, because every foreign key pointing at this entity resolves itself on
    /// delete rather than blocking one.
    pub async fn products_family_variants_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/family_variants/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Reads one family variant by its id — the whole row, every column, as it
    /// is stored.
    /// 
    /// A variant structure of a family: the attribute axes a product model splits
    /// its variants on — colour, then size. A product follows one through
    /// `family_variant_id`, and an attribute named as an axis becomes read-only on
    /// the model and is set on each variant instead, which is what `GET
    /// /products/attribute-schema` reports as `readonly_reason`. Two axis shapes
    /// are in the wild and both are read: a bare list of codes, or one entry per
    /// level.
    /// 
    /// An id no family variant of this tenant carries answers 404, and so does one
    /// belonging to another tenant: row-level security makes that row invisible
    /// rather than forbidden. A malformed id answers 400 before the route is
    /// reached.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_family_variants_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/family_variants/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Updates one family variant by id. A partial patch: the body names only the
    /// columns to change and every column it leaves out keeps its current value,
    /// so there is no read-modify-write and no way to blank a field by forgetting
    /// it.
    /// 
    /// A variant structure of a family: the attribute axes a product model splits
    /// its variants on — colour, then size. A product follows one through
    /// `family_variant_id`, and an attribute named as an axis becomes read-only on
    /// the model and is set on each variant instead, which is what `GET
    /// /products/attribute-schema` reports as `readonly_reason`. Two axis shapes
    /// are in the wild and both are read: a bare list of codes, or one entry per
    /// level.
    /// 
    /// A body that names nothing writable is refused with 400 rather than answered
    /// as a no-op, an id nobody carries answers 404, and a value that collides on
    /// `code` answers 409.
    pub async fn products_family_variants_update(&self, id: String, axes: Option<serde_json::Value>, code: Option<String>, family_id: Option<String>, labels: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/family_variants/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &axes {
            api_params.insert("axes".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &family_id {
            api_params.insert("family_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// A family of units and the standard one they all convert to — weight in
    /// kilograms, length in metres. A `measure` attribute names one and then
    /// offers exactly that family's units, and each unit's `convert_factor` is
    /// what makes two values recorded in different units comparable at all.
    /// 
    /// Every column of `measurement_families` is an exact-match query parameter,
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
    pub async fn products_measurement_families_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, code: Option<String>, standard_unit: Option<String>, units: Option<String>, labels: Option<String>, created_at: Option<String>, updated_at: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/measurement_families".to_string();

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
        if let Some(value) = &standard_unit {
            api_params.insert("standard_unit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &units {
            api_params.insert("units".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
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
    /// Creates one measurement family and answers 201 with the stored row,
    /// including the id and the timestamps the database filled in — a client
    /// never sends an id, it reads one back and uses it in the path of every later
    /// call.
    /// 
    /// A family of units and the standard one they all convert to — weight in
    /// kilograms, length in metres. A `measure` attribute names one and then
    /// offers exactly that family's units, and each unit's `convert_factor` is
    /// what makes two values recorded in different units comparable at all.
    /// 
    /// `code` and `standard_unit` are the only columns the database refuses the
    /// row without; everything else has a default or is nullable. A second row
    /// with the same `code` answers 409.
    pub async fn products_measurement_families_create(&self, code: String, standard_unit: String, labels: Option<serde_json::Value>, units: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/measurement_families".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("standard_unit".to_string(), serde_json::to_value(&standard_unit)?);
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &units {
            api_params.insert("units".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Deletes one measurement family by id. It is a hard delete — the row is
    /// gone, and the answer is a confirmation rather than a result to branch on.
    /// 
    /// Nothing in this schema references it, so nothing else changes.
    /// 
    /// An id no measurement family of this tenant carries answers 404; there is no
    /// 409, because every foreign key pointing at this entity resolves itself on
    /// delete rather than blocking one.
    pub async fn products_measurement_families_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/measurement_families/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Reads one measurement family by its id — the whole row, every column, as
    /// it is stored.
    /// 
    /// A family of units and the standard one they all convert to — weight in
    /// kilograms, length in metres. A `measure` attribute names one and then
    /// offers exactly that family's units, and each unit's `convert_factor` is
    /// what makes two values recorded in different units comparable at all.
    /// 
    /// An id no measurement family of this tenant carries answers 404, and so does
    /// one belonging to another tenant: row-level security makes that row
    /// invisible rather than forbidden. A malformed id answers 400 before the
    /// route is reached.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_measurement_families_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/measurement_families/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Updates one measurement family by id. A partial patch: the body names only
    /// the columns to change and every column it leaves out keeps its current
    /// value, so there is no read-modify-write and no way to blank a field by
    /// forgetting it.
    /// 
    /// A family of units and the standard one they all convert to — weight in
    /// kilograms, length in metres. A `measure` attribute names one and then
    /// offers exactly that family's units, and each unit's `convert_factor` is
    /// what makes two values recorded in different units comparable at all.
    /// 
    /// A body that names nothing writable is refused with 400 rather than answered
    /// as a no-op, an id nobody carries answers 404, and a value that collides on
    /// `code` answers 409.
    pub async fn products_measurement_families_update(&self, id: String, code: Option<String>, labels: Option<serde_json::Value>, standard_unit: Option<String>, units: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/measurement_families/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &standard_unit {
            api_params.insert("standard_unit".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &units {
            api_params.insert("units".to_string(), serde_json::to_value(value)?);
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
