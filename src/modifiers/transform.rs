use glam::{Affine3A, Vec3};

use crate::SdfResult;

pub fn transform(transform: Affine3A, sdf: Box<dyn Fn(Vec3) -> SdfResult>) -> Box<dyn Fn(Vec3) -> SdfResult> {
	let inverse_transform = transform.inverse();
	Box::new(move |pos| {
		let result = sdf(transform.transform_point3(pos));
		SdfResult {
			range: result.range,
			normal: inverse_transform.transform_vector3(result.normal).normalize_or_zero(),
			color: result.color
		}
	})
}
