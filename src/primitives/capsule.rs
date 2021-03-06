use glam::{Vec3, Affine3A};

use crate::{Color, SdfFn, union3, transform, truncated_cone, sphere, cylinder};

pub fn capsule(radius1: f32, radius2: f32, height: f32, color: Color) -> SdfFn {
	if radius1 == radius2 {
		union3(
			sphere(radius1, color),
			transform(
				Affine3A::from_translation(Vec3::Y * height),
				sphere(radius2, color),
			),
			cylinder(radius1, height, color),
		)
	} else {
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
		union3(
			sphere(radius1, color),
			transform(
				Affine3A::from_translation(Vec3::Y * height),
				sphere(radius2, color),
			),
			transform(
				Affine3A::from_translation(Vec3::Y * cone_offset1),
				truncated_cone(cone_radius1, cone_radius2, height - cone_offset1 + cone_offset2, color),
			)
		)
	}
}
