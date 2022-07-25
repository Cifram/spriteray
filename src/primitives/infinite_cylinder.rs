use glam::{Vec2, Vec3};

use crate::{SdfResult, Color, SdfFn};

pub fn infinite_cylinder(radius: f32, color: Color) -> SdfFn {
	Box::new(move |pos| {
		let pos2d = Vec2::new(pos.x, pos.z);
		let normal = Vec3::new(pos2d.x, 0.0, pos2d.y).normalize_or_zero();
		SdfResult { range: pos2d.length() - radius, normal, color }
	})
}
