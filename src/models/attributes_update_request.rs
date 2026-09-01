use serde::{Deserialize, Serialize};

/// Partial update — omitted fields keep their current value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributesUpdateRequest {
    /// The attribute's stable identifier — the KEY its value is stored under
    /// inside `attribute_values`, and the name a category rule addresses as
    /// `attribute:<code>`. Unique per (`entity_type`, `entity_ref`) in this
    /// tenant.
    #[serde(rename = "code", default)]
    pub code: String,
    /// Type-specific settings; which keys apply depends on `type`. The ones this
    /// app reads: `units` (the unit list a measure attribute offers) and
    /// `reference_entity` (which entity a reference attribute draws its options
    /// from). The ones the cockpit edits alongside them: `unit`, `metric_family`,
    /// `decimals_allowed`, `asset_family`, `max_file_size`, `allowed_extensions`.
    #[serde(rename = "config", default)]
    pub config: serde_json::Value,
    /// Narrows `entity_type` to ONE reference entity or asset family, by its code
    /// — the attributes of `brand` rather than of every reference entity. Null
    /// for a plain product attribute.
    #[serde(rename = "entity_ref", default)]
    pub entity_ref: String,
    /// Which kind of record carries this attribute: 'product' for the catalog
    /// itself, 'reference_entity', 'asset' or 'category' for the other things in
    /// this app that have attributes. Deliberately carries no CHECK — a tenant
    /// that models a fifth kind is served on it too.
    #[serde(rename = "entity_type", default)]
    pub entity_type: String,
    /// The `attribute_groups` row this attribute is filed under — the form
    /// section it appears in. Null is ungrouped, and an ungrouped field is
    /// rendered after every section that has a name.
    #[serde(rename = "group_id", default)]
    pub group_id: String,
    /// Offer this attribute as a filter in a product list. `GET /products/grid`
    /// reports exactly these attributes in its `filters` array, and nothing else
    /// reads the flag.
    #[serde(rename = "is_filterable", default)]
    pub is_filterable: bool,
    /// Declares that the value identifies the product — an EAN, a manufacturer
    /// part number. It is metadata a form and an importer read: no database index
    /// enforces it, because the value lives inside jsonb rather than in a column.
    #[serde(rename = "is_unique", default)]
    pub is_unique: bool,
    /// The field label a person sees, keyed by language tag. Resolution falls back
    /// to English and then to the code, so an untranslated attribute is still
    /// renderable.
    #[serde(rename = "labels", default)]
    pub labels: serde_json::Value,
    /// True → the record holds ONE VALUE PER LOCALE, under
    /// `attribute_values.locale_specific.<locale>.<code>`. False → one value,
    /// under `attribute_values.common.<code>`. This flag is what decides where a
    /// write goes.
    #[serde(rename = "localizable", default)]
    pub localizable: bool,
    /// Where the field sits inside its group. A family may override it for its own
    /// form through `family_attributes.position`; this is the attribute's default.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// True → one value PER CHANNEL, under
    /// `attribute_values.channel_specific.<channel>.<code>`. Set together with
    /// `localizable` it means one value per channel AND locale, in
    /// `channel_locale_specific`.
    #[serde(rename = "scopable", default)]
    pub scopable: bool,
    /// Which editor the value asks for — 'text', 'select', 'metric', 'price',
    /// 'asset_collection', 'reference_entity'. Carries no CHECK on purpose: an
    /// integrator adds a type, and `GET /products/attribute-schema` maps an
    /// unknown one onto a text field rather than refusing to answer.
    #[serde(rename = "type", default)]
    pub xtype: String,
    /// Show this attribute as a COLUMN in the product grid. `GET /products/grid`
    /// returns a column definition and a per-row value for exactly these.
    #[serde(rename = "usable_in_grid", default)]
    pub usable_in_grid: bool,
    /// Limits a value has to satisfy, as a flat object. The seven keys a client
    /// can act on are `min`, `max`, `min_length`, `max_length`, `pattern`,
    /// `min_items`, `max_items` — `GET /products/attribute-schema` republishes
    /// those and leaves anything else the tenant stored untouched.
    #[serde(rename = "validation", default)]
    pub validation: serde_json::Value,
}
