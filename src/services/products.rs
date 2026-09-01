use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// Products service
pub struct Products {
    client: Client,
}

impl Products {
    pub fn new(client: Client) -> Self {
        Products { client }
    }
    /// The catalog itself. A product row carries only what every product has —
    /// SKU, kind, family, enabled, tax class — and everything the tenant
    /// modelled lives in the `attribute_values` jsonb document, keyed by attribute
    /// CODE inside one of four scope buckets (common, per locale, per channel, per
    /// channel and locale). `label` is a generated column, maintained by the
    /// database so a grid of twenty thousand rows can sort and filter on a name
    /// with no join. `kind` says where the row sits in the variant hierarchy: a
    /// `model` carries what its variants share and is never sold itself.
    /// 
    /// Every column of `products` is an exact-match query parameter, `order` sorts
    /// by one column, and `limit`/`offset` page through `page.total`. A query key
    /// that is NOT a column is dropped rather than refused, and the `filter`
    /// object echoes the ones that were understood — that echo is the only way
    /// to tell an unfiltered answer from an empty one. It reads rows exactly as
    /// they are stored: no join is resolved, no jsonb value is unpacked, and
    /// soft-deleted products are included — filter on `deleted_at` to read the
    /// live catalog, or use `GET /products/grid`, which excludes them.
    pub async fn products_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, sku: Option<String>, kind: Option<String>, parent_id: Option<String>, family_id: Option<String>, family_variant_id: Option<String>, enabled: Option<bool>, tax_class: Option<String>, attribute_values: Option<String>, label: Option<String>, quantified_associations: Option<String>, completeness: Option<String>, created_at: Option<String>, updated_at: Option<String>, deleted_at: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products".to_string();

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
        if let Some(value) = &sku {
            api_params.insert("sku".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &kind {
            api_params.insert("kind".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &parent_id {
            api_params.insert("parent_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &family_id {
            api_params.insert("family_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &family_variant_id {
            api_params.insert("family_variant_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tax_class {
            api_params.insert("tax_class".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &attribute_values {
            api_params.insert("attribute_values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &label {
            api_params.insert("label".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantified_associations {
            api_params.insert("quantified_associations".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &completeness {
            api_params.insert("completeness".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &created_at {
            api_params.insert("created_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &updated_at {
            api_params.insert("updated_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &deleted_at {
            api_params.insert("deleted_at".to_string(), serde_json::to_value(value)?);
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
    /// Creates one product and answers 201 with the stored row, including the id
    /// and the timestamps the database filled in — a client never sends an id,
    /// it reads one back and uses it in the path of every later call.
    /// 
    /// The catalog itself. A product row carries only what every product has —
    /// SKU, kind, family, enabled, tax class — and everything the tenant
    /// modelled lives in the `attribute_values` jsonb document, keyed by attribute
    /// CODE inside one of four scope buckets (common, per locale, per channel, per
    /// channel and locale). `label` is a generated column, maintained by the
    /// database so a grid of twenty thousand rows can sort and filter on a name
    /// with no join. `kind` says where the row sits in the variant hierarchy: a
    /// `model` carries what its variants share and is never sold itself.
    /// 
    /// `sku` is the only column the database refuses the row without; everything
    /// else has a default or is nullable. A second row with the same `sku` answers
    /// 409. This app owns the create: `enabled` defaults from the
    /// `new_products_enabled_by_default` tenant setting rather than blindly to
    /// true, so an import cannot publish twenty thousand unfinished products the
    /// moment it lands, and a product that names no family gets the
    /// `default_product_family` one. An explicit value in the body always wins
    /// over both.
    pub async fn products_create(&self, sku: String, attribute_values: Option<serde_json::Value>, completeness: Option<serde_json::Value>, deleted_at: Option<String>, enabled: Option<bool>, family_id: Option<String>, family_variant_id: Option<String>, kind: Option<String>, parent_id: Option<String>, quantified_associations: Option<serde_json::Value>, tax_class: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("sku".to_string(), serde_json::to_value(&sku)?);
        if let Some(value) = &attribute_values {
            api_params.insert("attribute_values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &completeness {
            api_params.insert("completeness".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &deleted_at {
            api_params.insert("deleted_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &family_id {
            api_params.insert("family_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &family_variant_id {
            api_params.insert("family_variant_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &kind {
            api_params.insert("kind".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &parent_id {
            api_params.insert("parent_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantified_associations {
            api_params.insert("quantified_associations".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tax_class {
            api_params.insert("tax_class".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Answers four fields — id, sku, tax_class and the resolved display name
    /// — for a list of ids and/or SKUs in ONE call. It exists for the app on the
    /// other side of a product reference: the prices app holds SKUs and needs a
    /// tax class, a feed builder holds ids and needs names, and neither should
    /// page through the catalog or fire a request per line. Ask by either
    /// identifier or both; the two are unioned and a product named twice comes
    /// back once.
    /// 
    /// It answers what it FOUND: an id or SKU that names nothing is simply absent
    /// from `items` rather than an error, so compare the length of what you sent
    /// with what came back if a miss matters. It is not a general product read —
    /// for the whole row use `GET /products/{id}`, and for a scannable list use
    /// `GET /products/grid`.
    pub async fn products_batch(&self, ids: Option<Vec<String>>, skus: Option<Vec<String>>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/batch".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &ids {
            api_params.insert("ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &skus {
            api_params.insert("skus".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        if api_response.body.is_empty() {
            Ok(serde_json::Value::Null)
        } else {
            Ok(serde_json::from_slice(&api_response.body)?)
        }
    }
    /// The list a merchant can actually scan, as opposed to `GET /products`, which
    /// answers SKUs and a jsonb blob. Every row arrives already flattened: its
    /// resolved display name and where that name came from, its family code, its
    /// stored completeness, and the value of every attribute the catalog marks
    /// `usable_in_grid` — no join, no second call. `q` is a case-insensitive
    /// substring of the stored `label` column, which falls back to the SKU, so one
    /// box finds a product by either. Soft-deleted products are excluded here,
    /// unlike `GET /products`.
    /// 
    /// It filters on `q`, `kind`, `enabled` and `family_id`, and on NOTHING ELSE
    /// — a query parameter it does not accept is refused with 400 rather than
    /// dropped. That matters because of `filters`: the array reports the
    /// attributes marked `is_filterable`, which is what a filter bar should OFFER,
    /// and it is not a query surface. Filtering on an attribute value is not
    /// offered by this API at all — the values live inside a four-bucket jsonb
    /// document and are read through a fallback chain, so it is a feature with a
    /// design of its own rather than a parameter that was forgotten.
    pub async fn products_grid(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, q: Option<String>, kind: Option<String>, enabled: Option<bool>, family_id: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/grid".to_string();

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
        if let Some(value) = &q {
            api_params.insert("q".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &kind {
            api_params.insert("kind".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &family_id {
            api_params.insert("family_id".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// What is this product CALLED? A product's name is an attribute rather than a
    /// column, and which attribute it is, is per family — so no plain read can
    /// answer it. This resolves up to 500 products at once, by id and/or SKU: it
    /// reads families.label_attribute (falling back to the default_label_attribute
    /// setting, then to the conventional `name`) and looks the value up through
    /// the scoped attribute_values document — common, then locale_specific in
    /// the label_locales order, then the channel buckets.
    /// 
    /// It reports WHERE the name was found, which is the half that matters:
    /// `source: "sku"` means the catalog holds no name for this product and the
    /// SKU is standing in for one, so show it as a missing name rather than as a
    /// name. Writes nothing, and answers only what it found.
    pub async fn products_labels(&self, ids: Option<Vec<String>>, skus: Option<Vec<String>>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/labels".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &ids {
            api_params.insert("ids".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &skus {
            api_params.insert("skus".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// One relation from one product to another, of a declared type: this drill's
    /// accessories, this bundle's parts, this article's cross-sells. `quantity` is
    /// the number in "this bundle contains 4 casters" and is meaningful only when
    /// the association type carries `is_quantified`. This relational surface is
    /// the one this app serves; the `products.quantified_associations` column is
    /// an importer's blob that no route here reads or writes.
    /// 
    /// Every column of `product_associations` is an exact-match query parameter,
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
    pub async fn products_product_associations_list(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, id: Option<String>, product_id: Option<String>, association_type_id: Option<String>, target_product_id: Option<String>, quantity: Option<f64>, position: Option<i64>, created_at: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/product_associations".to_string();

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
        if let Some(value) = &association_type_id {
            api_params.insert("association_type_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &target_product_id {
            api_params.insert("target_product_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantity {
            api_params.insert("quantity".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
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
    /// Creates one product association and answers 201 with the stored row,
    /// including the id and the timestamps the database filled in — a client
    /// never sends an id, it reads one back and uses it in the path of every later
    /// call.
    /// 
    /// One relation from one product to another, of a declared type: this drill's
    /// accessories, this bundle's parts, this article's cross-sells. `quantity` is
    /// the number in "this bundle contains 4 casters" and is meaningful only when
    /// the association type carries `is_quantified`. This relational surface is
    /// the one this app serves; the `products.quantified_associations` column is
    /// an importer's blob that no route here reads or writes.
    /// 
    /// `product_id`, `association_type_id`, `target_product_id` are the only
    /// columns the database refuses the row without; everything else has a default
    /// or is nullable. A second row with the same `product_id`,
    /// `association_type_id`, `target_product_id` answers 409.
    pub async fn products_product_associations_create(&self, association_type_id: String, product_id: String, target_product_id: String, position: Option<i64>, quantity: Option<f64>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/product_associations".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("association_type_id".to_string(), serde_json::to_value(&association_type_id)?);
        api_params.insert("product_id".to_string(), serde_json::to_value(&product_id)?);
        api_params.insert("target_product_id".to_string(), serde_json::to_value(&target_product_id)?);
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantity {
            api_params.insert("quantity".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Deletes one product association by id. It is a hard delete — the row is
    /// gone, and the answer is a confirmation rather than a result to branch on.
    /// 
    /// Nothing in this schema references it, so nothing else changes.
    /// 
    /// An id no product association of this tenant carries answers 404; there is
    /// no 409, because every foreign key pointing at this entity resolves itself
    /// on delete rather than blocking one.
    pub async fn products_product_associations_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/product_associations/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Reads one product association by its id — the whole row, every column, as
    /// it is stored.
    /// 
    /// One relation from one product to another, of a declared type: this drill's
    /// accessories, this bundle's parts, this article's cross-sells. `quantity` is
    /// the number in "this bundle contains 4 casters" and is meaningful only when
    /// the association type carries `is_quantified`. This relational surface is
    /// the one this app serves; the `products.quantified_associations` column is
    /// an importer's blob that no route here reads or writes.
    /// 
    /// An id no product association of this tenant carries answers 404, and so
    /// does one belonging to another tenant: row-level security makes that row
    /// invisible rather than forbidden. A malformed id answers 400 before the
    /// route is reached.
    /// 
    /// Answered from the gateway's tenant cache for up to 30 minutes and dropped
    /// the moment this entity is written, because the data model changes weekly at
    /// most and every product page asks the same question.
    pub async fn products_product_associations_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/product_associations/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Updates one product association by id. A partial patch: the body names only
    /// the columns to change and every column it leaves out keeps its current
    /// value, so there is no read-modify-write and no way to blank a field by
    /// forgetting it.
    /// 
    /// One relation from one product to another, of a declared type: this drill's
    /// accessories, this bundle's parts, this article's cross-sells. `quantity` is
    /// the number in "this bundle contains 4 casters" and is meaningful only when
    /// the association type carries `is_quantified`. This relational surface is
    /// the one this app serves; the `products.quantified_associations` column is
    /// an importer's blob that no route here reads or writes.
    /// 
    /// A body that names nothing writable is refused with 400 rather than answered
    /// as a no-op, an id nobody carries answers 404, and a value that collides on
    /// `product_id`, `association_type_id`, `target_product_id` answers 409.
    pub async fn products_product_associations_update(&self, id: String, association_type_id: Option<String>, position: Option<i64>, product_id: Option<String>, quantity: Option<f64>, target_product_id: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/product_associations/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &association_type_id {
            api_params.insert("association_type_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &position {
            api_params.insert("position".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &product_id {
            api_params.insert("product_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantity {
            api_params.insert("quantity".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &target_product_id {
            api_params.insert("target_product_id".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The index of the enums this app ENFORCES — `product-kinds`,
    /// `membership-sources`, `rule-matches`, `asset-sources` — served by the app
    /// that owns the CHECK constraint each one is parsed out of, so a UI never has
    /// to keep its own copy of a status map and watch it drift. Names and titles
    /// only: fetch one by name for its values, badge tones and descriptions.
    /// 
    /// The set is a fixed property of this app rather than tenant data, so it is
    /// the same list for every tenant. `attributes.type` is deliberately absent:
    /// it carries no CHECK, because the whole point of an attribute-driven PIM is
    /// that the type list is data an integrator extends.
    pub async fn products_vocabularies_list(&self) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/products/vocabularies".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();

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
    /// One vocabulary with every value it admits, each with a title, a description
    /// and the badge tone a UI should paint it in. The value set is parsed out of
    /// the CHECK constraint in schema.json, so what is served IS what is enforced.
    /// Labels are curated on top and can only add words and colour — a permitted
    /// value nobody labelled still appears, titled from its own key.
    pub async fn products_vocabularies_get(&self, name: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/vocabularies/{name}".replace("{name}", &name.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("name".to_string(), serde_json::to_value(&name)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Deletes one product by id. It is a hard delete — the row is gone, and the
    /// answer is a confirmation rather than a result to branch on.
    /// 
    /// It takes what hangs off it: product category memberships (`product_id`),
    /// product associations (`product_id` and `target_product_id`) are deleted
    /// with it. `products.parent_id` is set to null instead, so the rows that
    /// pointed at it survive the delete rather than going with it.
    /// 
    /// An id no product of this tenant carries answers 404; there is no 409,
    /// because every foreign key pointing at this entity resolves itself on delete
    /// rather than blocking one. `products.deleted_at` is a SOFT-delete marker
    /// that the grid and every category-rule evaluation honour, but no route in
    /// this app ever writes it — to soft-delete instead, `PUT /products/{id}`
    /// with a `deleted_at`.
    pub async fn products_delete(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("DELETE", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Reads one product by its id — the whole row, every column, as it is
    /// stored.
    /// 
    /// The catalog itself. A product row carries only what every product has —
    /// SKU, kind, family, enabled, tax class — and everything the tenant
    /// modelled lives in the `attribute_values` jsonb document, keyed by attribute
    /// CODE inside one of four scope buckets (common, per locale, per channel, per
    /// channel and locale). `label` is a generated column, maintained by the
    /// database so a grid of twenty thousand rows can sort and filter on a name
    /// with no join. `kind` says where the row sits in the variant hierarchy: a
    /// `model` carries what its variants share and is never sold itself.
    /// 
    /// An id no product of this tenant carries answers 404, and so does one
    /// belonging to another tenant: row-level security makes that row invisible
    /// rather than forbidden. A malformed id answers 400 before the route is
    /// reached. Nothing is resolved for you here — for the display name, the
    /// family code and the grid attributes already unpacked, use `GET
    /// /products/grid` or `POST /products/labels`.
    pub async fn products_get(&self, id: String) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Updates one product by id. A partial patch: the body names only the columns
    /// to change and every column it leaves out keeps its current value, so there
    /// is no read-modify-write and no way to blank a field by forgetting it.
    /// 
    /// The catalog itself. A product row carries only what every product has —
    /// SKU, kind, family, enabled, tax class — and everything the tenant
    /// modelled lives in the `attribute_values` jsonb document, keyed by attribute
    /// CODE inside one of four scope buckets (common, per locale, per channel, per
    /// channel and locale). `label` is a generated column, maintained by the
    /// database so a grid of twenty thousand rows can sort and filter on a name
    /// with no join. `kind` says where the row sits in the variant hierarchy: a
    /// `model` carries what its variants share and is never sold itself.
    /// 
    /// A body that names nothing writable is refused with 400 rather than answered
    /// as a no-op, an id nobody carries answers 404, and a value that collides on
    /// `sku` answers 409. `label` is a generated column: naming it is dropped
    /// rather than refused, and `completeness` is written by the two metadata
    /// routes, not here.
    pub async fn products_update(&self, id: String, attribute_values: Option<serde_json::Value>, completeness: Option<serde_json::Value>, deleted_at: Option<String>, enabled: Option<bool>, family_id: Option<String>, family_variant_id: Option<String>, kind: Option<String>, parent_id: Option<String>, quantified_associations: Option<serde_json::Value>, sku: Option<String>, tax_class: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/{id}".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &attribute_values {
            api_params.insert("attribute_values".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &completeness {
            api_params.insert("completeness".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &deleted_at {
            api_params.insert("deleted_at".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &enabled {
            api_params.insert("enabled".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &family_id {
            api_params.insert("family_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &family_variant_id {
            api_params.insert("family_variant_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &kind {
            api_params.insert("kind".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &parent_id {
            api_params.insert("parent_id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &quantified_associations {
            api_params.insert("quantified_associations".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &sku {
            api_params.insert("sku".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &tax_class {
            api_params.insert("tax_class".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("PUT", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// How much of what its family REQUIRES does this product actually carry —
    /// the number a merchandiser works down. products.completeness is jsonb that
    /// nothing had ever written. This computes it from family_attributes
    /// (is_required) against the product's own scoped attribute_values and stores
    /// the result. A product with no family answers 400 rather than an invented 0
    /// % — it has nothing to be measured against.
    pub async fn products_completeness(&self, id: String, data: serde_json::Value) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/{id}/completeness".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        api_params.insert("data".to_string(), serde_json::to_value(&data)?);

        let mut api_headers: HashMap<String, String> = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let api_response = self
            .client
            .call("POST", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// Names the family in the body — by `family_id` or by `family_code`,
    /// whichever the caller holds — and computes the product's completeness in
    /// the same call. The step every family-driven surface waits on: a product
    /// with no family has no required attributes, so its completeness cannot be
    /// computed and its family's label attribute never resolves. Assigning the
    /// family recomputes and STORES products.completeness immediately, so the
    /// metadata cannot go stale between the two operations.
    pub async fn products_family_assign(&self, id: String, family_code: Option<String>, family_id: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/products/{id}/family".replace("{id}", &id.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("id".to_string(), serde_json::to_value(&id)?);
        if let Some(value) = &family_code {
            api_params.insert("family_code".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &family_id {
            api_params.insert("family_id".to_string(), serde_json::to_value(value)?);
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
