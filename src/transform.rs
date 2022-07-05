use glam::Affine3A;

use crate::{Sdf, SdfResult};

pub struct Transform<SdfT: Sdf> {
	transform: Affine3A,
	sdf: SdfT,
}

impl<SdfT: Sdf> Transform<SdfT> {
	pub fn new(transform: Affine3A, sdf: SdfT) -> Self {
		Self { transform, sdf }
	}
}

impl<SdfT: Sdf> Sdf for Transform<SdfT> {
	fn check(&self, pos: glam::Vec3) -> SdfResult {
		let result = self.sdf.check(self.transform.transform_point3(pos));
		SdfResult {
			range: result.range,
			normal: self.transform.transform_vector3(result.normal),
			color: result.color,
		}
	}
}
