use glam::Vec3;

use crate::SdfResult;

pub fn union<F1, F2>(pos: Vec3, a: F1, b: F2) -> SdfResult
	where
		F1: Fn(Vec3) -> SdfResult,
		F2: Fn(Vec3) -> SdfResult,
{
	match (a(pos), b(pos)) {
		(SdfResult::Miss { range: range1 }, SdfResult::Miss { range: range2 }) => SdfResult::Miss { range: range1.min(range2) },
		(SdfResult::Hit { range, color }, SdfResult::Miss { range: _ }) => SdfResult::Hit { range, color },
		(SdfResult::Miss { range: _ }, SdfResult::Hit { range, color }) => SdfResult::Hit { range, color },
		(SdfResult::Hit { range: range1, color: color1 }, SdfResult::Hit { range: range2, color: color2 }) => {
			if range1 > range2 {
				SdfResult::Hit { range: range1, color: color1 }
			} else {
				SdfResult::Hit { range: range2, color: color2 }
			}
		}
	}
}
