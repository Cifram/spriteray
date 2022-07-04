use glam::Vec3;

use crate::{SdfResult, Color};

pub fn torus(pos: Vec3, inner_radius: f32, outer_radius: f32, color: Color) -> SdfResult {
	if pos == Vec3::ZERO {
		return SdfResult {
			range: inner_radius,
			normal: Vec3::ZERO,
			color,
		}
	}

	let flat_dir = Vec3::new(pos.x, 0.0, pos.z).normalize();
	let center = flat_dir * (outer_radius + inner_radius) / 2.0;
	let radius = outer_radius - inner_radius;
	let offset = pos - center;
	let range = offset.length() - radius;
	SdfResult {
		range,
		normal: offset.normalize_or_zero(),
		color,
	}
}
