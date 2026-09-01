use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// ProductsReferences service
pub struct ProductsReferences {
    client: Client,
}

impl ProductsReferences {
    pub fn new(client: Client) -> Self {
        ProductsReferences { client }
    }
    /// A domain of records the catalog POINTS AT instead of duplicating —
    /// brands, manufacturers, care instructions. Declaring one is how a brand
    /// comes to be edited in one place rather than on nine thousand products. A
    /// reference entity has attributes of its own (`attributes` rows with
    /// `entity_type: "reference_entity"` and this entity's code as `entity_ref`),
    /// which is what makes its records more than a label.
    /// 
    /// Every column of `reference_entities` is an exact-match query parameter,
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
    pub async fn products_reference_entities_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, code: Option<String>, labels: Option<String>, image: Option<String>, created_at: Option<String>, updated_at: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/reference_entities".to_string();

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
        if let Some(value) = &image {
            api_params.insert("image".to_string(), serde_json::to_value(value)?);
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
    /// Creates one reference entity and answers 201 with the stored row, including
    /// the id and the timestamps the database filled in — a client never sends
    /// an id, it reads one back and uses it in the path of every later call.
    /// 
    /// A domain of records the catalog POINTS AT instead of duplicating —
    /// brands, manufacturers, care instructions. Declaring one is how a brand
    /// comes to be edited in one place rather than on nine thousand products. A
    /// reference entity has attributes of its own (`attributes` rows with
    /// `entity_type: "reference_entity"` and this entity's code as `entity_ref`),
    /// which is what makes its records more than a label.
    /// 
    /// `code` is the only column the database refuses the row without; everything
    /// else has a default or is nullable. A second row with the same `code`
    /// answers 409.
    pub async fn products_reference_entities_create(&self, code: String, image: Option<String>, labels: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/reference_entities".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        if let Some(value) = &image {
            api_params.insert("image".to_string(), serde_json::to_value(value)?);
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
    /// Deletes one reference entity by id. It is a hard delete — the row is
    /// gone, and the answer is a confirmation rather than a result to branch on.
    /// 
    /// It takes what hangs off it: reference entity records
    /// (`reference_entity_id`) are deleted with it.
    /// 
    /// An id no reference entity of this tenant carries answers 404; there is no
    /// 409, because every foreign key pointing at this entity resolves itself on
    /// delete rather than blocking one.
    pub async fn products_reference_entities_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/reference_entities/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Reads one reference entity by its id — the whole row, every column, as it
    /// is stored.
    /// 
    /// A domain of records the catalog POINTS AT instead of duplicating —
    /// brands, manufacturers, care instructions. Declaring one is how a brand
    /// comes to be edited in one place rather than on nine thousand products. A
    /// reference entity has attributes of its own (`attributes` rows with
    /// `entity_type: "reference_entity"` and this entity's code as `entity_ref`),
    /// which is what makes its records more than a label.
    /// 
    /// An id no reference entity of this tenant carries answers 404, and so does
    /// one belonging to another tenant: row-level security makes that row
    /// invisible rather than forbidden. A malformed id answers 400 before the
    /// route is reached.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_reference_entities_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/reference_entities/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Updates one reference entity by id. A partial patch: the body names only
    /// the columns to change and every column it leaves out keeps its current
    /// value, so there is no read-modify-write and no way to blank a field by
    /// forgetting it.
    /// 
    /// A domain of records the catalog POINTS AT instead of duplicating —
    /// brands, manufacturers, care instructions. Declaring one is how a brand
    /// comes to be edited in one place rather than on nine thousand products. A
    /// reference entity has attributes of its own (`attributes` rows with
    /// `entity_type: "reference_entity"` and this entity's code as `entity_ref`),
    /// which is what makes its records more than a label.
    /// 
    /// A body that names nothing writable is refused with 400 rather than answered
    /// as a no-op, an id nobody carries answers 404, and a value that collides on
    /// `code` answers 409.
    pub async fn products_reference_entities_update(&self, id: String, code: Option<String>, image: Option<String>, labels: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/reference_entities/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &image {
            api_params.insert("image".to_string(), serde_json::to_value(value)?);
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
    /// One record of a reference entity — one brand, one manufacturer. A product
    /// that points at it stores this record's CODE, exactly the way a select
    /// stores an option code, and the record's own properties live in its scoped
    /// `attribute_values` document. `GET /products/attribute-schema` offers these
    /// records as the `options` of any attribute that points at their entity, so a
    /// picker needs no second call.
    /// 
    /// Every column of `reference_entity_records` is an exact-match query
    /// parameter, `order` sorts by one column, and `limit`/`offset` page through
    /// `page.total`. A query key that is NOT a column is dropped rather than
    /// refused, and the `filter` object echoes the ones that were understood —
    /// that echo is the only way to tell an unfiltered answer from an empty one.
    /// It reads rows exactly as they are stored: no join is resolved, no jsonb
    /// value is unpacked.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_reference_entity_records_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, reference_entity_id: Option<String>, code: Option<String>, labels: Option<String>, attribute_values: Option<String>, created_at: Option<String>, updated_at: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/reference_entity_records".to_string();

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
        if let Some(value) = &reference_entity_id {
            api_params.insert("reference_entity_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &attribute_values {
            api_params.insert("attribute_values".to_string(), serde_json::to_value(value)?);
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
    /// Creates one reference entity record and answers 201 with the stored row,
    /// including the id and the timestamps the database filled in — a client
    /// never sends an id, it reads one back and uses it in the path of every later
    /// call.
    /// 
    /// One record of a reference entity — one brand, one manufacturer. A product
    /// that points at it stores this record's CODE, exactly the way a select
    /// stores an option code, and the record's own properties live in its scoped
    /// `attribute_values` document. `GET /products/attribute-schema` offers these
    /// records as the `options` of any attribute that points at their entity, so a
    /// picker needs no second call.
    /// 
    /// `reference_entity_id` and `code` are the only columns the database refuses
    /// the row without; everything else has a default or is nullable. A second row
    /// with the same `reference_entity_id` and `code` answers 409.
    pub async fn products_reference_entity_records_create(&self, code: String, reference_entity_id: String, attribute_values: Option<serde_json::Value>, labels: Option<serde_json::Value>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/reference_entity_records".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("code".to_string(), serde_json::to_value(&code)?);
        api_params.insert("reference_entity_id".to_string(), serde_json::to_value(&reference_entity_id)?);
        if let Some(value) = &attribute_values {
            api_params.insert("attribute_values".to_string(), serde_json::to_value(value)?);
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
    /// Deletes one reference entity record by id. It is a hard delete — the row
    /// is gone, and the answer is a confirmation rather than a result to branch
    /// on.
    /// 
    /// Nothing in this schema references it, so nothing else changes.
    /// 
    /// An id no reference entity record of this tenant carries answers 404; there
    /// is no 409, because every foreign key pointing at this entity resolves
    /// itself on delete rather than blocking one.
    pub async fn products_reference_entity_records_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/reference_entity_records/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Reads one reference entity record by its id — the whole row, every
    /// column, as it is stored.
    /// 
    /// One record of a reference entity — one brand, one manufacturer. A product
    /// that points at it stores this record's CODE, exactly the way a select
    /// stores an option code, and the record's own properties live in its scoped
    /// `attribute_values` document. `GET /products/attribute-schema` offers these
    /// records as the `options` of any attribute that points at their entity, so a
    /// picker needs no second call.
    /// 
    /// An id no reference entity record of this tenant carries answers 404, and so
    /// does one belonging to another tenant: row-level security makes that row
    /// invisible rather than forbidden. A malformed id answers 400 before the
    /// route is reached.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_reference_entity_records_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/reference_entity_records/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Updates one reference entity record by id. A partial patch: the body names
    /// only the columns to change and every column it leaves out keeps its current
    /// value, so there is no read-modify-write and no way to blank a field by
    /// forgetting it.
    /// 
    /// One record of a reference entity — one brand, one manufacturer. A product
    /// that points at it stores this record's CODE, exactly the way a select
    /// stores an option code, and the record's own properties live in its scoped
    /// `attribute_values` document. `GET /products/attribute-schema` offers these
    /// records as the `options` of any attribute that points at their entity, so a
    /// picker needs no second call.
    /// 
    /// A body that names nothing writable is refused with 400 rather than answered
    /// as a no-op, an id nobody carries answers 404, and a value that collides on
    /// `reference_entity_id` and `code` answers 409.
    pub async fn products_reference_entity_records_update(&self, id: String, attribute_values: Option<serde_json::Value>, code: Option<String>, labels: Option<serde_json::Value>, reference_entity_id: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/reference_entity_records/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &attribute_values {
            api_params.insert("attribute_values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &code {
            api_params.insert("code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &labels {
            api_params.insert("labels".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &reference_entity_id {
            api_params.insert("reference_entity_id".to_string(), serde_json::to_value(value)?);
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
