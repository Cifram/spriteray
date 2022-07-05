use glam::Vec3;

use crate::SdfResult;

pub trait Sdf {
	fn check(&self, pos: Vec3) -> SdfResult;
}
