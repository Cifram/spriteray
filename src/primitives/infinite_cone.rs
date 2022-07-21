use glam::{Vec2, Vec3};

use crate::{SdfResult, Color, Line, SdfFn};

pub fn infinite_cone(radius: f32, height: f32, color: Color) -> SdfFn {
	Box::new(move |pos| {
		let profile_pos = Vec2::new(Vec2::new(pos.x, pos.z).length(), pos.y);
		let hypot_normal = Vec2::new(height, radius).normalize();
		let hypot = Line::from_point_normal(Vec2::new(radius, 0.0), hypot_normal);
		let upper_bound = Line::from_point_normal(Vec2::new(0.0, height), Vec2::new(-hypot_normal.y, hypot_normal.x));
		if upper_bound.range(profile_pos) > 0.0 {
			SdfResult {
				range: profile_pos.distance(Vec2::new(0.0, height)),
				normal: (pos - Vec3::Y*height).normalize_or_zero(),
				color
			}
		} else {
			let rim_normal = Vec3::new(pos.x, 0.0, pos.z).normalize_or_zero();
			let hypot_range = hypot.range(profile_pos);
			SdfResult {
				range: hypot_range,
				normal: if pos.y < -hypot_range {
					Vec3::NEG_Y
				} else {
					rim_normal * hypot_normal.x + Vec3::Y * hypot_normal.y
				},
				color,
			}
		}
	})
}
