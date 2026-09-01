use serde::{Deserialize, Serialize};

/// One renderable field. A superset of the manifest's `Field`: the three
/// additions (`localized`, `channel_scoped`, `storage`) carry what a static
/// manifest never has to say, because a manifest's fields are columns and
/// these are keys inside one.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttributeField {
    /// One value per channel rather than one value.
    #[serde(rename = "channel_scoped", default)]
    pub channel_scoped: bool,
    /// Dotted read paths, most specific first — the documented precedence
    /// (channel+locale → locale → channel → common). `common` is always last
    /// and always present, because early imports wrote there whatever the
    /// attribute's flags say.
    #[serde(rename = "from", default)]
    pub from: Vec<String>,
    /// Attribute-group code — the section this field belongs in.
    #[serde(rename = "group", default)]
    pub group: String,
    /// That section's heading, resolved for the requested locale — so a form can
    /// be built without reading `attribute_groups` as well.
    #[serde(rename = "group_label", default)]
    pub group_label: String,
    /// Resolved for the requested locale, falling back to English, then to the
    /// code.
    #[serde(rename = "label", default)]
    pub label: String,
    /// One value per locale rather than one value.
    #[serde(rename = "localized", default)]
    pub localized: bool,
    /// The attribute code — the key the value is stored under.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Present on select / multi-select. Two sources, one shape: rows of
    /// `attribute_options` for an enumeration the attribute owns, or the records
    /// of a reference entity for an attribute that points at one. Empty is an
    /// answer: the list has no members yet.
    #[serde(rename = "options", default)]
    pub options: Vec<crate::models::AttributeFieldOption>,
    /// The family's ordering of this attribute, falling back to the attribute's
    /// own.
    #[serde(rename = "position", default)]
    pub position: i64,
    /// The field must not be edited in this context. Today the one cause is a
    /// variant axis on a product model; `readonly_reason` says which.
    #[serde(rename = "readonly", default)]
    pub readonly: bool,
    /// Why the field is locked — a variant axis on a product model is set on its
    /// variants.
    #[serde(rename = "readonly_reason", default)]
    pub readonly_reason: String,
    /// Present when the options ARE a reference entity's records: the code of that
    /// entity, so a client can offer to manage the values rather than only pick
    /// from them.
    #[serde(rename = "reference_entity", default)]
    pub reference_entity: String,
    /// The family's `is_required`, narrowed to the requested channel when
    /// `required_channels` names any.
    #[serde(rename = "required", default)]
    pub required: bool,
    /// Where the value lives. Absent on an app whose custom fields are plain
    /// columns — then the field name IS the column.
    #[serde(rename = "storage", default)]
    pub storage: crate::models::AttributeFieldStorage,
    /// The control to draw. Mapped from `attributes.type`, which carries no CHECK
    /// on purpose — an unknown type answers 'text' rather than nothing.
    #[serde(rename = "type", default)]
    pub xtype: String,
    /// The attribute's `is_unique` — the value is meant to identify the product.
    /// Advisory: no index enforces it, so a client that cares has to check.
    #[serde(rename = "unique", default)]
    pub unique: bool,
    /// Offered units of a `measure` field, from the attribute's `config.units`.
    #[serde(rename = "units", default)]
    pub units: Vec<String>,
    /// The limits the value has to satisfy, ready to hand to a form validator.
    /// Only the seven keys below are republished; anything else the tenant stored
    /// in `attributes.validation` stays there.
    #[serde(rename = "validation", default)]
    pub validation: crate::models::AttributeFieldValidation,
}
