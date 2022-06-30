use crate::Color;

pub enum SdfResult {
	Miss { range: f32 },
	Hit { range: f32, color: Color }
}
