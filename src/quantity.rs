use super::*;

#[derive(Clone, Debug)]
pub struct Quantity {
    int: Int64Amount,
    format: Format,
}

impl Quantity {
    pub fn new(value: i64, format: Format) -> Self {
        let int = Int64Amount::new(value);
        Self { int, format }
    }

    pub fn milli(value: i64, format: Format) -> Self {
        let int = Int64Amount::with_scale(value, Scale::Milli, format);
        Self { int, format }
    }

    pub fn scaled(value: i64, scale: Scale) -> Self {
        let format = Format::DecimalSI;
        let int = Int64Amount::with_scale(value, scale, format);
        Self { int, format }
    }

    pub fn value(&self) -> i64 {
        // self.int.as_scaled_int64(Scale::None).unwrap_or(i64::MAX)
        self.int.raw_value()
    }

    pub fn as_int64(&self) -> Option<i64> {
        self.int.as_int64()
    }

    pub fn to_text(&self) -> Option<String> {
        self.int.to_text(self.format)
    }
}

impl str::FromStr for Quantity {
    type Err = InvalidQuantity;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let length = text.len();

        // First check two-letter suffixes
        if length > 2 {
            let (number, suffix) = text.split_at(length - 2);
            if let Ok(scale) = suffix.parse() {
                let format = Format::BinarySI;
                if let Ok(value) = number.parse() {
                    let int = Int64Amount::with_scale(value, scale, format);
                    return Ok(Self { int, format });
                }
            }
        }

        // Then check one-letter suffixes
        if length > 1 {
            let (number, suffix) = text.split_at(length - 1);
            if let Ok(scale) = suffix.parse() {
                let format = Format::DecimalSI;
                if let Ok(value) = number.parse() {
                    let int = Int64Amount::with_scale(value, scale, format);
                    return Ok(Self { int, format });
                }
            }
        }

        // TODO Add check for exponential notation 'e'|'E'

        // And finally try without suffix
        let format = Format::DecimalSI;
        let value = text.parse()?;
        let int = Int64Amount::with_scale(value, Scale::None, format);
        Ok(Self { int, format })
    }
}

impl From<Quantity> for Option<i64> {
    fn from(value: Quantity) -> Self {
        value.into()
    }
}

impl From<Quantity> for Option<u64> {
    fn from(value: Quantity) -> Self {
        value.as_int64()?.try_into().ok()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidQuantity(num::ParseIntError);

impl From<num::ParseIntError> for InvalidQuantity {
    fn from(value: num::ParseIntError) -> Self {
        Self(value)
    }
}

impl fmt::Display for InvalidQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl error::Error for InvalidQuantity {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.0)
    }
}
