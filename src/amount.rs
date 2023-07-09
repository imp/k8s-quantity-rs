use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Int64Amount {
    value: i64,
    scale: Scale,
}

impl Int64Amount {
    pub(super) fn new(value: i64) -> Self {
        Self::with_scale(value, Scale::None, Format::DecimalSI)
    }

    pub(super) fn with_scale(value: i64, scale: Scale, format: Format) -> Self {
        let value = if let Some(multiplier) = scale.format_multiplier(format) {
            value * multiplier
        } else {
            value
        };
        Self { value, scale }
    }

    pub(super) fn raw_value(&self) -> i64 {
        self.value
    }

    pub fn as_int64(&self) -> Option<i64> {
        self.scale.integer_multiplier()?.checked_mul(self.value)
    }

    pub fn as_scale(&self, _scale: Scale) -> Self {
        todo!()
    }

    pub fn as_scaled_int64(&self, scale: Scale) -> Option<i64> {
        if self.scale == scale {
            self.as_int64()
        } else if self.scale < scale {
            // larger scale
            todo!()
        } else {
            // smaller scale
            todo!()
        }
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
