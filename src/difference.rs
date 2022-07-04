use glam::Vec3;

use crate::{SdfResult, union2, union3, union4, union5};

pub fn difference2<F1, F2>(pos: Vec3, a: &F1, b: &F2) -> SdfResult
	where
		F1: Fn(Vec3) -> SdfResult,
		F2: Fn(Vec3) -> SdfResult,
{
	let a = a(pos);
	let b = b(pos);
	if a.range > 0.0 {
		// It missed the positive shape, and thus is definitely outside.
		// Use the result from the positive shape.
		a
	} else if a.range <= 0.0 && b.range > 0.0 {
		// It hit the positive shape and missed the negative shape, and thus
		// this is a hit. Use the results of the surface it's closest to, but
		// if that's the negative shape it has to invert it.
		if -a.range < b.range {
			a
		} else {
			SdfResult {
				range: -b.range,
				normal: -b.normal,
				color: b.color,
			}
		}
	} else {
		// It hit both shapes, so it's in the negative carve-out. Use the
		// result from the negative shape, but inverted.
		SdfResult {
			range: -b.range,
			normal: -b.normal,
			color: b.color,
		}
	}
}

pub fn difference3<F1, F2, F3>(pos: Vec3, a: &F1, b: &F2, c: &F3) -> SdfResult
	where
		F1: Fn(Vec3) -> SdfResult,
		F2: Fn(Vec3) -> SdfResult,
		F3: Fn(Vec3) -> SdfResult,
{
	difference2(pos, a, &|point| union2(point, b, c))
}

pub fn difference4<F1, F2, F3, F4>(pos: Vec3, a: &F1, b: &F2, c: &F3, d: &F4) -> SdfResult
	where
		F1: Fn(Vec3) -> SdfResult,
		F2: Fn(Vec3) -> SdfResult,
		F3: Fn(Vec3) -> SdfResult,
		F4: Fn(Vec3) -> SdfResult,
{
	difference2(pos, a, &|point| union3(point, b, c, d))
}

pub fn difference5<F1, F2, F3, F4, F5>(pos: Vec3, a: &F1, b: &F2, c: &F3, d: &F4, e: &F5) -> SdfResult
	where
		F1: Fn(Vec3) -> SdfResult,
		F2: Fn(Vec3) -> SdfResult,
		F3: Fn(Vec3) -> SdfResult,
		F4: Fn(Vec3) -> SdfResult,
		F5: Fn(Vec3) -> SdfResult,
{
	difference2(pos, a, &|point| union4(point, b, c, d, e))
}

pub fn difference6<F1, F2, F3, F4, F5, F6>(pos: Vec3, a: &F1, b: &F2, c: &F3, d: &F4, e: &F5, f: &F6) -> SdfResult
	where
		F1: Fn(Vec3) -> SdfResult,
		F2: Fn(Vec3) -> SdfResult,
		F3: Fn(Vec3) -> SdfResult,
		F4: Fn(Vec3) -> SdfResult,
		F5: Fn(Vec3) -> SdfResult,
		F6: Fn(Vec3) -> SdfResult,
{
	difference2(pos, a, &|point| union5(point, b, c, d, e, f))
}
