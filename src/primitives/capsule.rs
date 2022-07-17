use glam::{Vec3, Affine3A};

use crate::{SdfResult, Color, Sphere, TruncatedCone, Union3, Transform, Sdf};

pub struct Capsule {
	sdf: Union3<Sphere, Transform<Sphere>, Transform<TruncatedCone>>,
}

impl Capsule {
	pub fn new(radius1: f32, radius2: f32, height: f32, color: Color) -> Self {
		let mut tan_angle = std::f32::consts::PI / 2.0 - ((radius1 - radius2).abs() / height).acos();
		if radius1 < radius2 {
			tan_angle = -tan_angle;
		}
		let cos_tan_angle = tan_angle.cos();
		let sin_tan_angle = tan_angle.sin();
		let cone_radius1 = radius1 * cos_tan_angle;
		let cone_radius2 = radius2 * cos_tan_angle;
		let cone_offset1 = radius1 * sin_tan_angle;
		let cone_offset2 = radius2 * sin_tan_angle;
		Self {
			sdf: Union3::new(
				Sphere::new(radius1, color),
				Transform::new(
					Affine3A::from_translation(Vec3::NEG_Y * height),
					Sphere::new(radius2, color),
				),
				Transform::new(
					Affine3A::from_translation(Vec3::NEG_Y * cone_offset1),
					TruncatedCone::new(cone_radius1, cone_radius2, height - cone_offset1 + cone_offset2, color),
				)
			)
		}
	}
}

impl Sdf for Capsule {
	fn check(&self, pos: Vec3) -> SdfResult {
		self.sdf.check(pos)
	}
}
