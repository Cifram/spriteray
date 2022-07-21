use crate::{Color, SdfFn, difference2, infinite_cone, plane};

pub fn cone(radius: f32, height: f32, color: Color) -> SdfFn {
	difference2(
		infinite_cone(radius, height, color),
		plane(color),
	)
}
