use crate::SdfFn;

pub fn intersection2(a: SdfFn, b: SdfFn) -> SdfFn {
	Box::new(move |pos| {
		let a = a(pos);
		let b = b(pos);
		if a.range > b.range { a } else { b }
	})
}

pub fn intersection3(a: SdfFn, b: SdfFn, c: SdfFn) -> SdfFn {
	intersection2(a, intersection2(b, c))
}

pub fn intersection4(a: SdfFn, b: SdfFn, c: SdfFn, d: SdfFn) -> SdfFn {
	intersection2(a, intersection3(b, c, d))
}

pub fn intersection5(a: SdfFn, b: SdfFn, c: SdfFn, d: SdfFn, e: SdfFn) -> SdfFn {
	intersection2(a, intersection4(b, c, d, e))
}

pub fn intersection6(a: SdfFn, b: SdfFn, c: SdfFn, d: SdfFn, e: SdfFn, f: SdfFn) -> SdfFn {
	intersection2(a, intersection5(b, c, d, e, f))
}
