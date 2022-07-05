use glam::Vec3;

use crate::{SdfResult, Sdf, Union2, Union3, Union4, Union5};

pub struct Difference2<SdfT1: Sdf, SdfT2: Sdf> {
	a: SdfT1,
	b: SdfT2,
}

impl<SdfT1: Sdf, SdfT2: Sdf> Difference2<SdfT1, SdfT2> {
	pub fn new(a: SdfT1, b: SdfT2) -> Self {
		Self { a, b }
	}
}

impl<SdfT1: Sdf, SdfT2: Sdf> Sdf for Difference2<SdfT1, SdfT2> {
	fn check(&self, pos: Vec3) -> SdfResult {
		let a = self.a.check(pos);
		let b = self.b.check(pos);
		if a.range > -b.range {
			a
		} else {
			SdfResult {
				range: -b.range,
				normal: -b.normal,
				color: b.color,
			}
		}
	}
}

pub struct Difference3<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf> {
	sdf: Difference2<SdfT1, Union2<SdfT2, SdfT3>>,
}

impl<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf> Difference3<SdfT1, SdfT2, SdfT3> {
	pub fn new(a: SdfT1, b: SdfT2, c: SdfT3) -> Self {
		Self { sdf: Difference2::new(a, Union2::new(b, c)) }
	}
}

impl<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf> Sdf for Difference3<SdfT1, SdfT2, SdfT3> {
	fn check(&self, pos: Vec3) -> SdfResult {
		self.sdf.check(pos)
	}
}

pub struct Difference4<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf, SdfT4: Sdf> {
	sdf: Difference2<SdfT1, Union3<SdfT2, SdfT3, SdfT4>>,
}

impl<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf, SdfT4: Sdf> Difference4<SdfT1, SdfT2, SdfT3, SdfT4> {
	pub fn new(a: SdfT1, b: SdfT2, c: SdfT3, d: SdfT4) -> Self {
		Self { sdf: Difference2::new(a, Union3::new(b, c, d)) }
	}
}

impl<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf, SdfT4: Sdf> Sdf for Difference4<SdfT1, SdfT2, SdfT3, SdfT4> {
	fn check(&self, pos: Vec3) -> SdfResult {
		self.sdf.check(pos)
	}
}

pub struct Difference5<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf, SdfT4: Sdf, SdfT5: Sdf> {
	sdf: Difference2<SdfT1, Union4<SdfT2, SdfT3, SdfT4, SdfT5>>,
}

impl<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf, SdfT4: Sdf, SdfT5: Sdf> Difference5<SdfT1, SdfT2, SdfT3, SdfT4, SdfT5> {
	pub fn new(a: SdfT1, b: SdfT2, c: SdfT3, d: SdfT4, e: SdfT5) -> Self {
		Self { sdf: Difference2::new(a, Union4::new(b, c, d, e)) }
	}
}

impl<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf, SdfT4: Sdf, SdfT5: Sdf> Sdf for Difference5<SdfT1, SdfT2, SdfT3, SdfT4, SdfT5> {
	fn check(&self, pos: Vec3) -> SdfResult {
		self.sdf.check(pos)
	}
}

pub struct Difference6<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf, SdfT4: Sdf, SdfT5: Sdf, SdfT6: Sdf> {
	sdf: Difference2<SdfT1, Union5<SdfT2, SdfT3, SdfT4, SdfT5, SdfT6>>,
}

impl<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf, SdfT4: Sdf, SdfT5: Sdf, SdfT6: Sdf> Difference6<SdfT1, SdfT2, SdfT3, SdfT4, SdfT5, SdfT6> {
	pub fn new(a: SdfT1, b: SdfT2, c: SdfT3, d: SdfT4, e: SdfT5, f: SdfT6) -> Self {
		Self { sdf: Difference2::new(a, Union5::new(b, c, d, e, f)) }
	}
}

impl<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf, SdfT4: Sdf, SdfT5: Sdf, SdfT6: Sdf> Sdf for Difference6<SdfT1, SdfT2, SdfT3, SdfT4, SdfT5, SdfT6> {
	fn check(&self, pos: Vec3) -> SdfResult {
		self.sdf.check(pos)
	}
}
