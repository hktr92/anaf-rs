use serde::{Deserialize, Serialize};

use crate::ApiResponse;

#[cfg(feature = "vat_payer_async_api")]
pub type VatPayerAsyncResponse = crate::AsyncApiResponse<VatPayerAsyncToken>;

#[cfg(feature = "vat_payer_async_api")]
#[derive(Debug, Deserialize)]
pub struct VatPayerAsyncToken {
    #[serde(alias = "correlationId")]
    pub id: String,
}

#[cfg(feature = "vat_payer_async_api")]
impl std::fmt::Display for crate::vat_payer::VatPayerAsyncToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub type VatPayerResponse = ApiResponse<VatPayerResponseItem>;

#[derive(Debug, Deserialize, Serialize)]
pub struct VatPayerResponseItem {
    #[serde(alias = "date_generale")]
    pub company_info: CompanyInfo,

    #[serde(alias = "inregistrare_scop_Tva")]
    pub vat_scope: VatScope,

    #[serde(alias = "inregistrare_RTVAI")]
    pub vat_payer_income: VatPayerIncome,

    #[serde(alias = "stare_inactiv")]
    pub inactive: InactiveStatus,

    #[serde(alias = "inregistrare_SplitTVA")]
    pub vat_split: VatSplit,

    #[serde(alias = "adresa_sediu_social")]
    pub hq_address: Address,

    #[serde(alias = "adresa_domiciliu_fiscal")]
    pub fiscal_address: Address,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CompanyInfo {
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

    #[serde(alias = "data_inregistrare")]
    pub registration_date: String,

    #[serde(alias = "cod_CAEN")]
    pub activity_code: String,

    #[serde(alias = "iban")]
    pub iban: String,

    #[serde(alias = "statusRO_e_Factura")]
    pub has_ro_einvoice: bool,

    // added in v8
    #[serde(alias = "organFiscalCompetent")]
    pub trusted_fiscal_activity: Option<String>,

    // added in v8
    #[serde(alias = "forma_de_proprietate")]
    pub property_form: Option<String>,

    // added in v8
    #[serde(alias = "forma_organizare")]
    pub organization_form: Option<String>,

    // added in v8
    #[serde(alias = "forma_juriidica")]
    pub juridic_form: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VatScope {
    #[serde(alias = "scpTVA")]
    pub is_payer: bool,

    // added in v8
    #[serde(alias = "perioade_TVA", flatten)]
    pub payer_interval: VatPayerInterval,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VatPayerInterval {
    #[serde(alias = "data_inceput_ScpTVA")]
    pub from: Option<String>,

    #[serde(alias = "data_sfarsit_ScpTVA")]
    pub to: Option<String>,

    #[serde(alias = "data_anul_imp_ScpTVA")]
    pub cancelled_at: Option<String>,

    #[serde(alias = "mesaj_ScpTVA")]
    pub cancelled_reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VatPayerIncome {
    #[serde(alias = "dataInceputTvaInc")]
    pub from: String,
    #[serde(alias = "dataSfarsitTvaInc")]
    pub to: Option<String>,
    #[serde(alias = "dataActualizareTvaInc")]
    pub updated_at: Option<String>,
    #[serde(alias = "dataPublicareTvaInc")]
    pub published_at: Option<String>,
    #[serde(alias = "tipActTvaInc")]
    pub update_type: String,
    #[serde(alias = "statusTvaIncasare")]
    pub status: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InactiveStatus {
    #[serde(alias = "dataInactivare")]
    pub deactivated_at: String,
    #[serde(alias = "dataReactivare")]
    pub reactivated_at: String,
    #[serde(alias = "dataPublicare")]
    pub published_at: String,
    #[serde(alias = "dataRadiere")]
    pub erased_at: String,
    #[serde(alias = "statusInactivi")]
    pub status: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VatSplit {
    #[serde(alias = "dataInceputSplitTVA")]
    pub started_at: String,

    #[serde(alias = "dataAnulareSplitTVA")]
    pub cancelled_at: String,

    #[serde(alias = "statusSplitTVA")]
    pub status: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    #[serde(alias = "sdenumire_Strada", alias = "ddenumire_Strada")]
    pub street: String,

    #[serde(alias = "snumar_Strada", alias = "dnumar_Strada")]
    pub number: String,

    #[serde(alias = "sdenumire_Localitate", alias = "ddenumire_Localitate")]
    pub town: String,

    #[serde(alias = "scod_Localitate", alias = "dcod_Localitate")]
    pub town_code: String,

    #[serde(alias = "sdenumire_Judet", alias = "ddenumire_Judet")]
    pub county: String,

    #[serde(alias = "scod_Judet", alias = "dcod_Judet")]
    pub county_code: String,

    #[serde(alias = "scod_JudetAuto", alias = "dcod_JudetAuto")]
    pub county_code_auto: String,

    #[serde(alias = "stara", alias = "dtara")]
    pub country: String,

    #[serde(alias = "sdetalii_Adresa", alias = "ddetalii_Adresa")]
    pub details: String,

    #[serde(alias = "scod_Postal", alias = "dcod_Postal")]
    pub postal_code: String,
}
