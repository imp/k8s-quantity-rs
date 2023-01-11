use super::*;

impl Quantity {
    pub fn decimal(value: i64, scale: Scale) -> Self {
        let int = Int64Amount::with_scale(value, scale);
        let format = Format::DecimalSI;
        Self { int, format }
    }

    pub fn as_int64(&self) -> Option<i64> {
        self.int.as_int64(self.format)
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
                    let int = Int64Amount::with_scale(value, scale);
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
                    let int = Int64Amount::with_scale(value, scale);
                    return Ok(Self { int, format });
                }
            }
        }

        // TODO Add check for exponential notation 'e'|'E'

        // And finally try without suffix
        let format = Format::DecimalSI;
        let value = text.parse()?;
        let int = Int64Amount::with_scale(value, Scale::None);
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
