use glam::{Vec2, Vec3};

use crate::{SdfResult, Color, Line, Sdf};

pub struct Cone {
	radius: f32,
	height: f32,
	color: Color,
}

impl Cone {
	pub fn new(radius: f32, height: f32, color: Color) -> Self {
		Self { radius, height, color }
	}
}

impl Sdf for Cone {
	fn check(&self, pos: Vec3) -> SdfResult {
		let flat_pos = Vec2::new(Vec2::new(pos.x, pos.z).length(), pos.y);
		if flat_pos.x < self.radius && flat_pos.y < 0.0 {
			SdfResult {
				range: -flat_pos.y,
				normal: Vec3::NEG_Y,
				color: self.color,
			}
		} else {
			let hypot_normal = Vec2::new(self.height, self.radius).normalize();
			let hypot = Line::from_point_normal(Vec2::new(self.radius, 0.0), hypot_normal);
			let lower_bound = Line::from_point_normal(Vec2::new(self.radius, 0.0), Vec2::new(hypot_normal.y, -hypot_normal.x));
			let upper_bound = Line::from_point_normal(Vec2::new(0.0, self.height), Vec2::new(-hypot_normal.y, hypot_normal.x));
			if lower_bound.range(flat_pos) > 0.0 {
				let rim_point = Vec3::new(pos.x, 0.0, pos.z).normalize_or_zero() * self.radius;
				SdfResult {
					range: flat_pos.distance(Vec2::new(self.radius, 0.0)),
					normal: (pos - rim_point).normalize_or_zero(),
					color: self.color
				}
			} else if upper_bound.range(flat_pos) > 0.0 {
				SdfResult {
					range: flat_pos.distance(Vec2::new(0.0, self.height)),
					normal: (pos - Vec3::Y*self.height).normalize_or_zero(),
					color: self.color
				}
			} else {
				let rim_normal = Vec3::new(pos.x, 0.0, pos.z).normalize_or_zero();
				let hypot_range = hypot.range(flat_pos);
				SdfResult {
					range: hypot_range,
					normal: if pos.y < -hypot_range {
						Vec3::NEG_Y
					} else {
						rim_normal * hypot_normal.x + Vec3::Y * hypot_normal.y
					},
					color: self.color,
				}
			}
		}
	}
}
