use glam::Vec3;

use crate::{Color, SdfFn, infinite_cylinder, difference3, plane_normal_offset};

pub fn cylinder(radius: f32, height: f32, color: Color) -> SdfFn {
	difference3(
		infinite_cylinder(radius, color),
		plane_normal_offset(Vec3::Y, 0.0, color),
		plane_normal_offset(Vec3::NEG_Y, height, color),
	)
}
