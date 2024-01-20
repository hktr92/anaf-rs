use std::fmt::Display;

#[derive(Debug, Default, Clone)]
pub enum BalanceApiVersion {
    #[default]
    V1,
}

impl BalanceApiVersion {
    pub fn latest() -> Self {
        Self::default()
    }
}

impl Display for BalanceApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BalanceApiVersion::V1 => "v1",
            }
        )
    }
}
