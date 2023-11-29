use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BalanceRequest {
    #[serde(rename = "cui")]
    pub registration_code: usize,

    #[serde(rename = "an")]
    pub year: usize,
}

impl BalanceRequest {
    pub fn new(registration_code: usize, year: usize) -> Self {
        Self {
            registration_code,
            year,
        }
    }
}
