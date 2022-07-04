use glam::Vec3;

use crate::{SdfResult, Color, union3, sphere, truncated_cone, mirror_y};

pub fn capsule(pos: Vec3, radius1: f32, radius2: f32, height: f32, color: Color) -> SdfResult {
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
	union3(pos,
		&|pos| sphere(pos, radius1, color),
		&|pos| sphere(pos - Vec3::Y * height, radius2, color),
		&|pos| if cone_radius1 < cone_radius2 {
			mirror_y(pos, |pos| truncated_cone(
				pos + Vec3::Y * (height + cone_offset2),
				cone_radius2, cone_radius1, height - cone_offset2 + cone_offset1, color
			))
		} else {
			truncated_cone(
				pos - Vec3::Y * cone_offset1,
				cone_radius1, cone_radius2, height - cone_offset1 + cone_offset2, color
			)
		}
	)
}
