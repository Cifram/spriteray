use glam::{Vec2, Vec3};

use crate::{SdfResult, Color, Line};

pub fn cone(pos: Vec3, radius: f32, height: f32, color: Color) -> SdfResult {
	let flat_pos = Vec2::new(Vec2::new(pos.x, pos.z).length(), pos.y);
	if flat_pos.x < radius && flat_pos.y < 0.0 {
		SdfResult::Miss { range: -flat_pos.y }
	} else {
		let hypot_normal = Vec2::new(height, radius).normalize();
		let hypot = Line::from_point_normal(Vec2::new(radius, 0.0), hypot_normal);
		let lower_bound = Line::from_point_normal(Vec2::new(radius, 0.0), Vec2::new(hypot_normal.y, -hypot_normal.x));
		let upper_bound = Line::from_point_normal(Vec2::new(0.0, height), Vec2::new(-hypot_normal.y, hypot_normal.x));
		if lower_bound.range(flat_pos) > 0.0 {
			SdfResult::Miss { range: flat_pos.distance(Vec2::new(radius, 0.0)) }
		} else if upper_bound.range(flat_pos) > 0.0 {
			SdfResult::Miss { range: flat_pos.distance(Vec2::new(0.0, height)) }
		} else {
			let range = hypot.range(flat_pos);
			if range > 0.0 {
				SdfResult::Miss { range }
			} else {
				SdfResult::Hit { range, color }
			}
		}
	}
}
