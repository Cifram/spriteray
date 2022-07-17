use glam::{Vec3, Affine3A, Quat};

use crate::{SdfResult, Color, Sdf, Difference2, Cone, Plane, Transform};

pub struct TruncatedCone {
	sdf: Transform<Difference2<Cone, Transform<Plane>>>,
}

impl TruncatedCone {
	pub fn new(base_radius: f32, top_radius: f32, height: f32, color: Color) -> Self {
		let (base_radius, top_radius, transform) = if base_radius > top_radius {
			(base_radius, top_radius, Affine3A::IDENTITY)
		} else {
			(top_radius, base_radius, Affine3A::from_scale_rotation_translation(Vec3::new(1.0, -1.0, 1.0), Quat::IDENTITY, Vec3::Y * height))
		};
		let full_height = (height * base_radius) / (base_radius - top_radius);
		Self {
			sdf: Transform::new(
				transform,
				Difference2::new(
					Cone::new(base_radius, full_height, color),
					Transform::new(
						Affine3A::from_scale_rotation_translation(Vec3::NEG_Y, Quat::IDENTITY, Vec3::Y * height),
						Plane::new(color),
					),
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
