use glam::Vec3;

use crate::{SdfResult, Color, SdfFn};

pub fn plane(color: Color) -> SdfFn {
	Box::new(move |pos| {
		SdfResult { range: pos.y, normal: Vec3::Y, color }
	})
}
