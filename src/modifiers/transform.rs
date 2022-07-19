use glam::{Affine3A, Vec3};

use crate::SdfResult;

pub fn transform(transform: Affine3A, sdf: Box<dyn Fn(Vec3) -> SdfResult>) -> Box<dyn Fn(Vec3) -> SdfResult> {
	Box::new(move |pos| {
		let result = sdf(transform.transform_point3(pos));
		SdfResult {
			range: result.range,
			normal: transform.transform_vector3(result.normal),
			color: result.color
		}
	})
}
