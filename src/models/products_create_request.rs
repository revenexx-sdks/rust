use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProductsCreateRequest {
    /// Every attribute value the record carries, in ONE jsonb document — the
    /// core of an attribute-driven PIM. A record's properties are not columns
    /// here: they are rows in `attributes`, selected per family by
    /// `family_attributes`, and their values live under their attribute CODE
    /// inside this object.
    /// 
    /// Four buckets, and an attribute's own flags decide which one it writes to:
    /// 
    /// `common`                    the attribute is neither localizable nor
    /// scopable — one value, full stop.
    /// `{"common": {"net_weight": 2.4, "colour": "black"}}`
    /// `locale_specific`           `localizable`: one value per language tag.
    /// `{"locale_specific": {"de_DE": {"name": "Akku-Bohrschrauber"}}}`
    /// `channel_specific`          `scopable`: one value per channel.
    /// `{"channel_specific": {"b2b": {"minimum_order_quantity": 6}}}`
    /// `channel_locale_specific`   both: one value per channel AND language tag.
    /// `{"channel_locale_specific": {"b2b": {"de_DE": {"description": "…"}}}}`
    /// 
    /// A reader takes the most specific bucket that carries the code and falls
    /// back through locale, then channel, then `common`. `common` is always last
    /// and always consulted, because early imports wrote everything there whatever
    /// an attribute's flags said — a reader that skipped it reports an imported
    /// catalog as empty. `GET /products/attribute-schema` answers, per field, the
    /// exact path a value belongs at (`storage.path`) and that full fallback order
    /// (`from`), so no client has to re-derive any of this.
    /// 
    /// The value itself is whatever the attribute's `type` implies: a string, a
    /// number, a boolean, an option CODE for a select (never its label), a list of
    /// codes for a multi-select, `{"amount": …, "unit": …}` for a measure, a
    /// list of `{"amount": …, "currency": …}` for a price, an asset code for
    /// media.
    /// 
    /// Defaults to `{}`, and an empty object is a normal state — a record nobody
    /// has enriched yet. The declared type also admits an array only because every
    /// jsonb column of this app shares one mapping; an array is not meaningful
    /// here and every reader in this app treats a non-object as empty.
    #[serde(rename = "attribute_values", default)]
    pub attribute_values: serde_json::Value,
    /// How much of what this product's family REQUIRES it actually carries — the
    /// number a merchandiser works down. `required` counts the attributes the
    /// family marks `is_required`, `filled` how many of those carry a value in ANY
    /// bucket, `ratio` is filled/required between 0 and 1 (a family that requires
    /// nothing is 1, not undefined), `missing` lists the codes with no value
    /// anywhere, sorted, and `computed_at` is when it was measured.
    /// 
    /// Written only by `POST /products/{id}/completeness` and by `POST
    /// /products/{id}/family`; a plain create or update never touches it, so it is
    /// null until one of the two has run. It also stays null for a product with no
    /// family — there is nothing to measure it against, and 0 % would be a lie.
    #[serde(rename = "completeness", default)]
    pub completeness: serde_json::Value,
    /// When the product was soft-deleted. `GET /products/grid` and every
    /// category-rule evaluation exclude a row that carries one; `GET /products`
    /// does NOT — filter on it to read the live catalog.
    #[serde(rename = "deleted_at", default)]
    pub deleted_at: String,
    /// Whether the product is offered. A create defaults it from the
    /// `new_products_enabled_by_default` tenant setting rather than blindly to
    /// true, so an import does not publish twenty thousand unfinished products the
    /// moment it lands. An explicit value in the body always wins.
    #[serde(rename = "enabled", default)]
    pub enabled: bool,
    /// The family that decides which attributes this product HAS. Without one
    /// nothing is required, completeness cannot be computed and the display name
    /// never resolves — `POST /products/{id}/family` is the call that sets it
    /// and computes completeness in the same step.
    #[serde(rename = "family_id", default)]
    pub family_id: String,
    /// Which variant structure of the family this product follows — the axes it
    /// splits on. Null on a simple product.
    #[serde(rename = "family_variant_id", default)]
    pub family_variant_id: String,
    /// Where the product sits in the variant hierarchy. 'simple' stands on its
    /// own. 'model' carries the values its variants share and is never sold
    /// itself. 'variant' carries the axis values and points at its model through
    /// `parent_id`.
    #[serde(rename = "kind", default)]
    pub kind: String,
    /// The product MODEL this variant belongs to. Only a `variant` carries one.
    /// Deleting the model leaves its variants behind with a null parent rather
    /// than deleting them.
    #[serde(rename = "parent_id", default)]
    pub parent_id: String,
    /// The import-side mirror of associations that carry a quantity — a bundle,
    /// a bill of materials, a spare-parts set. NOTHING IN THIS APP READS OR WRITES
    /// IT: no route produces it, no route consumes it, and it is null on every
    /// product this app has created. The surface that IS served is relational —
    /// `product_associations`, whose `quantity` column holds the number, guarded
    /// by `association_types.is_quantified`.
    /// 
    /// It exists because a PIM import (Akeneo, BMEcat) carries these in one blob
    /// keyed by association type code, and the column lets that document
    /// round-trip instead of being dropped. The database enforces no shape on it,
    /// so what a reader finds is whatever the importer wrote; the example is the
    /// conventional form.
    #[serde(rename = "quantified_associations", default)]
    pub quantified_associations: serde_json::Value,
    /// The merchant's own article number — unique per tenant, and the value
    /// every integration (ERP, shop, feed, price list) joins on. The one
    /// identifier a person types, and the fallback this app shows when the catalog
    /// holds no name.
    #[serde(rename = "sku", default)]
    pub sku: String,
    /// The tax class key the prices app resolves a VAT rate from. Free text here
    /// — the vocabulary belongs to the app that prices, and `POST
    /// /products/batch` exists to hand exactly this column to it in bulk.
    #[serde(rename = "tax_class", default)]
    pub tax_class: String,
}
