use glam::Vec3;

use crate::SdfResult;

pub fn mirror_x<F>(pos: Vec3, sdf: F) -> SdfResult
	where F: Fn(Vec3) -> SdfResult
{
	let result = sdf(Vec3::new(-pos.x, pos.y, pos.z));
	SdfResult {
		range: result.range,
		normal: Vec3::new(-result.normal.x, result.normal.y, result.normal.z),
		color: result.color,
	}
}

pub fn mirror_y<F>(pos: Vec3, sdf: F) -> SdfResult
	where F: Fn(Vec3) -> SdfResult
{
	let result = sdf(Vec3::new(pos.x, -pos.y, pos.z));
	SdfResult {
		range: result.range,
		normal: Vec3::new(result.normal.x, -result.normal.y, result.normal.z),
		color: result.color,
	}
}

pub fn mirror_z<F>(pos: Vec3, sdf: F) -> SdfResult
	where F: Fn(Vec3) -> SdfResult
{
	let result = sdf(Vec3::new(pos.x, pos.y, -pos.z));
	SdfResult {
		range: result.range,
		normal: Vec3::new(result.normal.x, result.normal.y, -result.normal.z),
		color: result.color,
	}
}
