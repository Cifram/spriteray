use glam::{Affine3A, Vec3};

use crate::SdfResult;

pub fn transform(transform: Affine3A, sdf: Box<dyn Fn(Vec3) -> SdfResult>) -> Box<dyn Fn(Vec3) -> SdfResult> {
	let (scale, rotation, _) = transform.to_scale_rotation_translation();
	Box::new(move |pos| {
		let result = sdf(transform.transform_point3(pos));
		SdfResult {
			range: result.range,
			normal: (rotation.inverse() * result.normal) * scale,
			color: result.color
		}
	})
}
