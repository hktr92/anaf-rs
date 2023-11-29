use std::fmt::Display;

#[derive(Debug, Default, Clone)]
pub enum VatPayerApiVersion {
    #[default]
    V8,
    V7,
}

impl Display for VatPayerApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VatPayerApiVersion::V8 => "v8",
                VatPayerApiVersion::V7 => "v7",
            }
        )
    }
}

impl VatPayerApiVersion {
    pub fn latest() -> Self {
        Self::default()
    }
}
