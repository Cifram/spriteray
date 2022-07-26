use glam::Vec3;

use crate::{SdfResult, Color, SdfFn};

pub fn plane_normal_offset(normal: Vec3, offset: f32, color: Color) -> SdfFn {
	Box::new(move |pos| {
		SdfResult { range: pos.dot(normal) + offset, normal, color }
	})
}

pub fn plane_normal_point(normal: Vec3, point: Vec3, color: Color) -> SdfFn {
	let offset = -normal.dot(point);
	plane_normal_offset(normal, offset, color)
}

pub fn plane_three_points(a: Vec3, b: Vec3, c: Vec3, color: Color) -> SdfFn {
	let normal = (a - b).cross(a - c).normalize();
	plane_normal_point(normal, a, color)
}
