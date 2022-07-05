use glam::Vec3;

use crate::{SdfResult, Sdf};

pub struct Union2<SdfT1: Sdf, SdfT2: Sdf> {
	a: SdfT1,
	b: SdfT2,
}

impl<SdfT1: Sdf, SdfT2: Sdf> Union2<SdfT1, SdfT2> {
	pub fn new(a: SdfT1, b: SdfT2) -> Self {
		Self { a, b }
	}
}

impl<SdfT1: Sdf, SdfT2: Sdf> Sdf for Union2<SdfT1, SdfT2> {
	fn check(&self, pos: Vec3) -> SdfResult {
		let a = self.a.check(pos);
		let b = self.b.check(pos);
		if a.range < b.range { a } else { b }
	}
}

pub struct Union3<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf> {
	sdf: Union2<SdfT1, Union2<SdfT2, SdfT3>>,
}

impl<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf> Union3<SdfT1, SdfT2, SdfT3> {
	pub fn new(a: SdfT1, b: SdfT2, c: SdfT3) -> Self {
		Self { sdf: Union2::new(a, Union2::new(b, c)) }
	}
}

impl<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf> Sdf for Union3<SdfT1, SdfT2, SdfT3> {
	fn check(&self, pos: Vec3) -> SdfResult {
		self.sdf.check(pos)
	}
}

pub struct Union4<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf, SdfT4: Sdf> {
	sdf: Union2<SdfT1, Union2<SdfT2, Union2<SdfT3, SdfT4>>>,
}

impl<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf, SdfT4: Sdf> Union4<SdfT1, SdfT2, SdfT3, SdfT4> {
	pub fn new(a: SdfT1, b: SdfT2, c: SdfT3, d: SdfT4) -> Self {
		Self { sdf: Union2::new(a, Union2::new(b, Union2::new(c, d))) }
	}
}

impl<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf, SdfT4: Sdf> Sdf for Union4<SdfT1, SdfT2, SdfT3, SdfT4> {
	fn check(&self, pos: Vec3) -> SdfResult {
		self.sdf.check(pos)
	}
}

pub struct Union5<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf, SdfT4: Sdf, SdfT5: Sdf> {
	sdf: Union2<SdfT1, Union2<SdfT2, Union2<SdfT3, Union2<SdfT4, SdfT5>>>>,
}

impl<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf, SdfT4: Sdf, SdfT5: Sdf> Union5<SdfT1, SdfT2, SdfT3, SdfT4, SdfT5> {
	pub fn new(a: SdfT1, b: SdfT2, c: SdfT3, d: SdfT4, e: SdfT5) -> Self {
		Self { sdf: Union2::new(a, Union2::new(b, Union2::new(c, Union2::new(d, e)))) }
	}
}

impl<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf, SdfT4: Sdf, SdfT5: Sdf> Sdf for Union5<SdfT1, SdfT2, SdfT3, SdfT4, SdfT5> {
	fn check(&self, pos: Vec3) -> SdfResult {
		self.sdf.check(pos)
	}
}

pub struct Union6<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf, SdfT4: Sdf, SdfT5: Sdf, SdfT6: Sdf> {
	sdf: Union2<SdfT1, Union2<SdfT2, Union2<SdfT3, Union2<SdfT4, Union2<SdfT5, SdfT6>>>>>,
}

impl<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf, SdfT4: Sdf, SdfT5: Sdf, SdfT6: Sdf> Union6<SdfT1, SdfT2, SdfT3, SdfT4, SdfT5, SdfT6> {
	pub fn new(a: SdfT1, b: SdfT2, c: SdfT3, d: SdfT4, e: SdfT5, f: SdfT6) -> Self {
		Self { sdf: Union2::new(a, Union2::new(b, Union2::new(c, Union2::new(d, Union2::new(e, f))))) }
	}
}

impl<SdfT1: Sdf, SdfT2: Sdf, SdfT3: Sdf, SdfT4: Sdf, SdfT5: Sdf, SdfT6: Sdf> Sdf for Union6<SdfT1, SdfT2, SdfT3, SdfT4, SdfT5, SdfT6> {
	fn check(&self, pos: Vec3) -> SdfResult {
		self.sdf.check(pos)
	}
}
