use glam::Vec3;

use crate::{Color, SdfFn, difference2, infinite_cone, plane_normal_offset};

pub fn cone(radius: f32, height: f32, color: Color) -> SdfFn {
	difference2(
		infinite_cone(radius, height, color),
		plane_normal_offset(Vec3::Y, 0.0, color),
	)
}
