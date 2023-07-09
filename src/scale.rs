use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scale {
    Nano = -9,
    Micro = -6,
    Milli = -3,
    None = 0,
    Kilo = 3,
    Mega = 6,
    Giga = 9,
    Tera = 12,
    Peta = 15,
    Exa = 18,
}

impl Scale {
    pub(super) fn integer_multiplier(self) -> Option<i64> {
        self.decimal_multiplier()
    }

    pub(super) fn format_multiplier(self, format: Format) -> Option<i64> {
        match format {
            Format::DecimalExponent => todo!(),
            Format::BinarySI => self.binary_miltiplier(),
            Format::DecimalSI => self.decimal_multiplier(),
        }
    }

    fn binary_miltiplier(self) -> Option<i64> {
        match self {
            Self::None => Some(1),
            Self::Kilo => Some(1024),
            Self::Mega => Some(1024 * 1024),
            Self::Giga => Some(1024 * 1024 * 1024),
            Self::Tera => Some(1024 * 1024 * 1024 * 1024),
            Self::Peta => Some(1024 * 1024 * 1024 * 1024 * 1024),
            Self::Exa => Some(1024 * 1024 * 1024 * 1024 * 1024 * 1024),
            _ => None,
        }
    }

    fn decimal_multiplier(self) -> Option<i64> {
        match self {
            Self::None => Some(1),
            Self::Kilo => Some(1_000),
            Self::Mega => Some(1_000_000),
            Self::Giga => Some(1_000_000_000),
            Self::Tera => Some(1_000_000_000_000),
            Self::Peta => Some(1_000_000_000_000_000),
            Self::Exa => Some(1_000_000_000_000_000_000),
            _ => None,
        }
    }

    fn as_decimal(&self) -> &str {
        match self {
            Self::Nano => "n",
            Self::Micro => "µ",
            Self::Milli => "m",
            Self::None => "",
            Self::Kilo => "k",
            Self::Mega => "M",
            Self::Giga => "G",
            Self::Tera => "T",
            Self::Peta => "P",
            Self::Exa => "E",
        }
    }

    fn as_str_alternative(&self) -> &str {
        match self {
            Self::Nano => "n (10^-9)",
            Self::Micro => "µ (10^-6)",
            Self::Milli => "m (10^-3)",
            Self::None => "",
            Self::Kilo => "k (10^3)",
            Self::Mega => "M (10^6)",
            Self::Giga => "G (10^9)",
            Self::Tera => "T (10^12)",
            Self::Peta => "P (10^15)",
            Self::Exa => "E (10^18)",
        }
    }

    fn as_binary(&self) -> Option<&str> {
        match self {
            Self::Nano | Self::Micro | Self::Milli => None,
            Self::None => Some(""),
            Self::Kilo => Some("Ki"),
            Self::Mega => Some("Mi"),
            Self::Giga => Some("Gi"),
            Self::Tera => Some("Ti"),
            Self::Peta => Some("Pi"),
            Self::Exa => Some("Ei"),
        }
    }

    pub(crate) fn as_str_with_format(&self, format: Format) -> Option<&str> {
        match format {
            Format::DecimalExponent => todo!(),
            Format::BinarySI => self.as_binary(),
            Format::DecimalSI => Some(self.as_decimal()),
        }
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            self.as_str_alternative().fmt(f)
        } else {
            self.as_decimal().fmt(f)
        }
    }
}

impl str::FromStr for Scale {
    type Err = InvalidScale;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" => Ok(Self::Nano),
            "u" | "µ" => Ok(Self::Micro),
            "m" => Ok(Self::Milli),
            "k" | "Ki" => Ok(Self::Kilo),
            "M" | "Mi" => Ok(Self::Mega),
            "G" | "Gi" => Ok(Self::Giga),
            "T" | "Ti" => Ok(Self::Tera),
            "P" | "Pi" => Ok(Self::Peta),
            "E" | "Ei" => Ok(Self::Exa),
            _ => Err(InvalidScale),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InvalidScale;

impl fmt::Display for InvalidScale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "Invalid Scale".fmt(f)
    }
}

impl error::Error for InvalidScale {}
