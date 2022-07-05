use glam::{Vec3, Affine3A, Quat};

use crate::{SdfResult, Color, Sdf, Difference2, Cone, Plane, Transform};

pub struct TruncatedCone {
	sdf: Difference2<Cone, Transform<Plane>>
}

impl TruncatedCone {
	pub fn new(base_radius: f32, top_radius: f32, height: f32, color: Color) -> Self {
		let full_height = (height * base_radius) / (base_radius - top_radius);
		Self {
			sdf: Difference2::new(
				Cone::new(base_radius, full_height, color),
				Transform::new(
					Affine3A::from_scale_rotation_translation(Vec3::NEG_Y, Quat::IDENTITY, Vec3::Y * height),
					Plane::new(color),
				),
			)
		}
	}
}

impl Sdf for TruncatedCone {
	fn check(&self, pos: Vec3) -> SdfResult {
		self.sdf.check(pos)
	}
}
