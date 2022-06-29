use glam::Vec3;

use crate::Color;

pub enum SdfResult {
	Miss { range: f32 },
	Hit { range: f32, normal: Vec3, color: Color }
}
