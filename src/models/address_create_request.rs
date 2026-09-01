use serde::{Deserialize, Serialize};

/// An address needs an owner: 'organization_id' or 'contact_id'.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddressCreateRequest {
    /// City or town.
    #[serde(rename = "city", default)]
    pub city: String,
    /// Company line on the label. Often the owning organization's name, but not
    /// always — a delivery to a construction site carries the site.
    #[serde(rename = "company", default)]
    pub company: String,
    /// Owning person — a personal address only that contact uses. Exactly one of
    /// organization_id / contact_id is set.
    #[serde(rename = "contact_id", default)]
    pub contact_id: String,
    /// ISO 3166-1 alpha-2 country code, exactly two letters. Uppercase by
    /// convention; it is what shipping and tax both key off.
    #[serde(rename = "country", default)]
    pub country: String,
    /// The default address of its owner AND type: one default billing and one
    /// default shipping address per owner. Setting it moves the flag off the
    /// previous holder. Default false.
    #[serde(rename = "is_default", default)]
    pub is_default: bool,
    /// Recipient line on the label — the person or department the parcel is
    /// addressed to.
    #[serde(rename = "name", default)]
    pub name: String,
    /// Owning company — a company address, shared by everyone in it. Exactly one
    /// of organization_id / contact_id is set.
    #[serde(rename = "organization_id", default)]
    pub organization_id: String,
    /// Phone number for the carrier to reach at this address — often a different
    /// one from the contact's own.
    #[serde(rename = "phone", default)]
    pub phone: String,
    /// State, province or Bundesland. Required by some destinations (US, CA),
    /// unused by most European ones.
    #[serde(rename = "region", default)]
    pub region: String,
    /// Street and house number, on one line, as the local post expects it.
    #[serde(rename = "street", default)]
    pub street: String,
    /// The second address line: building, floor, gate, c/o. Null when there is
    /// none.
    #[serde(rename = "street2", default)]
    pub street2: String,
    /// What the address is FOR — one of the tenant's own address types (GET
    /// /customers/address-types), seeded with billing and shipping. A merchant may
    /// add their own (a works entrance, a central accounts office) without a
    /// release of this app. A create without it gets the type flagged as default;
    /// a type the tenant does not keep is a 400.
    #[serde(rename = "type", default)]
    pub xtype: String,
    /// Postal code, as text — leading zeros are real in most countries.
    #[serde(rename = "zip", default)]
    pub zip: String,
}
