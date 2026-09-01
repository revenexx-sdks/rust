use std::collections::HashMap;

use serde_json::Value;

use crate::client::Client;
use crate::error::Error;
use crate::input_file::InputFile;

/// PagesDelivery service
pub struct PagesDelivery {
    client: Client,
}

impl PagesDelivery {
    pub fn new(client: Client) -> Self {
        PagesDelivery { client }
    }
    /// One call gives a theme its whole chrome: header, footer and account
    /// navigation, each under the key the theme looks it up by. This route reads
    /// no filter — fetch all of them once and index by `id`.
    pub async fn pages_delivery_menus(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/delivery/menus".to_string();

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
    /// What a storefront calls to render a URL: `GET
    /// /pages/delivery/page?slug=about-us&langcode=de`. Send exactly one selector
    /// — `slug` or `id`. `slug` is matched against the page and then against its
    /// translations, so a localized URL resolves to its page. Only the PUBLISHED
    /// revision is served, so an edit in progress never leaks. What comes back is
    /// finished rather than raw: `langcode` is resolved field by field with the
    /// page's source language behind it, blocks whose publish window has not
    /// opened or has already closed are left out, and every library reference is
    /// expanded into the subtree it points at — so a renderer walks the tree it
    /// is given and makes no second call for any of it.
    pub async fn pages_delivery_page(&self, slug: Option<String>, id: Option<String>, langcode: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/delivery/page".to_string();

        let mut api_params: HashMap<String, Value> = HashMap::new();
        if let Some(value) = &slug {
            api_params.insert("slug".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &id {
            api_params.insert("id".to_string(), serde_json::to_value(value)?);
        }
        if let Some(value) = &langcode {
            api_params.insert("langcode".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
    /// The route a sitemap, a static build or a link picker is generated from.
    /// Only published pages, never a soft-deleted one — `filter` echoes both
    /// predicates the route applies on its own. A `?status=` of your own is
    /// ignored: this route is the published view by definition.
    pub async fn pages_delivery_pages(&self, limit: Option<i64>, offset: Option<i64>, order: Option<String>, bundle: Option<String>) -> Result<serde_json::Value, Error> {
        let api_path = "/v1/pages/delivery/pages".to_string();

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
        if let Some(value) = &bundle {
            api_params.insert("bundle".to_string(), serde_json::to_value(value)?);
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
    /// The same shape `GET /pages/delivery/page` answers, built from the
    /// UNPUBLISHED working copy instead of the published revision — so a
    /// reviewer without an editor account sees exactly what the storefront would
    /// render.
    pub async fn pages_delivery_preview(&self, token: String, langcode: Option<String>) -> Result<crate::models::Error, Error> {
        let api_path = "/v1/pages/delivery/preview/{token}".replace("{token}", &token.to_string());

        let mut api_params: HashMap<String, Value> = HashMap::new();
        api_params.insert("token".to_string(), serde_json::to_value(&token)?);
        if let Some(value) = &langcode {
            api_params.insert("langcode".to_string(), serde_json::to_value(value)?);
        }

        let mut api_headers: HashMap<String, String> = HashMap::new();

        let api_response = self
            .client
            .call("GET", &api_path, api_headers, api_params)
            .await?;

        Ok(serde_json::from_slice(&api_response.body)?)
    }
}
