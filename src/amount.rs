use super::*;

#[derive(Clone, Debug)]
pub struct Int64Amount {
    value: i64,
    scale: Scale,
}

impl Int64Amount {
    pub(super) fn with_scale(value: i64, scale: Scale) -> Self {
        Self { value, scale }
    }

    pub fn as_int64(&self, format: Format) -> Option<i64> {
        self.scale
            .integer_multiplier(format)?
            .checked_mul(self.value)
    }

    pub fn as_scaled(&self, _scale: Scale) -> Option<i64> {
        None
    }

    pub fn to_text(&self, format: Format) -> Option<String> {
        self.scale
            .as_str_with_format(format)
            .map(|suffix| format!("{}{suffix}", self.value))
    }
}

impl From<i64> for Int64Amount {
    fn from(value: i64) -> Self {
        let scale = Scale::None;
        Self { value, scale }
    }
}
