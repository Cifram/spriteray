use glam::Vec3;

use crate::SdfResult;

pub fn union2<F1, F2>(pos: Vec3, a: &F1, b: &F2) -> SdfResult
	where
		F1: Fn(Vec3) -> SdfResult,
		F2: Fn(Vec3) -> SdfResult,
{
	let a = a(pos);
	let b = b(pos);
	if a.range < b.range { a } else { b }
}

pub fn union3<F1, F2, F3>(pos: Vec3, a: &F1, b: &F2, c: &F3) -> SdfResult
	where
		F1: Fn(Vec3) -> SdfResult,
		F2: Fn(Vec3) -> SdfResult,
		F3: Fn(Vec3) -> SdfResult,
{
	union2(pos, a, &|pos| union2(pos, b, c))
}

pub fn union4<F1, F2, F3, F4>(pos: Vec3, a: &F1, b: &F2, c: &F3, d: &F4) -> SdfResult
	where
		F1: Fn(Vec3) -> SdfResult,
		F2: Fn(Vec3) -> SdfResult,
		F3: Fn(Vec3) -> SdfResult,
		F4: Fn(Vec3) -> SdfResult,
{
	union2(pos, a, &|pos| union3(pos, b, c, d))
}

pub fn union5<F1, F2, F3, F4, F5>(pos: Vec3, a: &F1, b: &F2, c: &F3, d: &F4, e: &F5) -> SdfResult
	where
		F1: Fn(Vec3) -> SdfResult,
		F2: Fn(Vec3) -> SdfResult,
		F3: Fn(Vec3) -> SdfResult,
		F4: Fn(Vec3) -> SdfResult,
		F5: Fn(Vec3) -> SdfResult,
{
	union2(pos, a, &|pos| union4(pos, b, c, d, e))
}

pub fn union6<F1, F2, F3, F4, F5, F6>(pos: Vec3, a: &F1, b: &F2, c: &F3, d: &F4, e: &F5, f: &F6) -> SdfResult
	where
		F1: Fn(Vec3) -> SdfResult,
		F2: Fn(Vec3) -> SdfResult,
		F3: Fn(Vec3) -> SdfResult,
		F4: Fn(Vec3) -> SdfResult,
		F5: Fn(Vec3) -> SdfResult,
		F6: Fn(Vec3) -> SdfResult,
{
	union2(pos, a, &|pos| union5(pos, b, c, d, e, f))
}
