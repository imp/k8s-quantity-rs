use super::*;

#[derive(Clone, Copy, Debug)]
pub enum Format {
    DecimalExponent, // e.g., 12e6
    BinarySI,        // e.g., 12Mi (12 * 2^20)
    DecimalSI,       // e.g., 12M  (12 * 10^6)
}

impl Format {
    pub fn as_str(&self) -> &str {
        match self {
            Self::DecimalExponent => "DecimalExponent",
            Self::BinarySI => "BinarySI",
            Self::DecimalSI => "DecimalSI",
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
