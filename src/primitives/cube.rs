use glam::Vec3;

use crate::{Color, SdfFn, plane_normal_offset, intersection6};

pub fn cube(size: f32, color: Color) -> SdfFn {
	intersection6(
		plane_normal_offset(Vec3::NEG_Y, 0.0, color),
		plane_normal_offset(Vec3::Y, -size, color),
		plane_normal_offset(Vec3::NEG_X, -size / 2.0, color),
		plane_normal_offset(Vec3::X, -size / 2.0, color),
		plane_normal_offset(Vec3::NEG_Z, -size / 2.0, color),
		plane_normal_offset(Vec3::Z, -size / 2.0, color),
	)
}
