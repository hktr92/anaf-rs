use serde::Deserialize;

pub type FarmerResponse = Vec<FarmerResponseItem>;

#[derive(Debug, Deserialize)]
pub struct FarmerResponseItem {
    #[serde(alias = "cui")]
    pub unique_registration_code: usize,

    #[serde(alias = "data")]
    pub when: String,

    #[serde(alias = "denumire")]
    pub name: String,

    #[serde(alias = "adresa")]
    pub address: String,

    #[serde(alias = "nrRegCom")]
    pub commerce_registry_number: String,

    #[serde(alias = "telefon")]
    pub phone: String,

    #[serde(alias = "fax")]
    pub fax: String,

    #[serde(alias = "codPostal")]
    pub postal_code: String,

    #[serde(alias = "act")]
    pub act: String,

    #[serde(alias = "stare_inregistrare")]
    pub registration_status: String,

    #[serde(alias = "dataInceputRegAgric")]
    pub farmer_since: String,

    #[serde(alias = "dataAnulareRegAgric")]
    pub farmer_until: Option<String>,

    #[serde(alias = "statusRegAgric")]
    pub is_active: bool,
}
