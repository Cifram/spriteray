use crate::{SdfResult, Color, SdfFn};

pub fn sphere(radius: f32, color: Color) -> SdfFn {
	Box::new(move |pos| SdfResult { range: pos.length() - radius, normal: pos.normalize_or_zero(), color })
}
