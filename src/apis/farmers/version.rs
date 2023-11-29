use std::fmt::Display;

#[derive(Debug, Clone, Default)]
pub enum FarmerApiVersion {
    #[default]
    V2,
}

impl Display for FarmerApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::V2 => "v2",
            }
        )
    }
}
