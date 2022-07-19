use crate::SdfFn;

pub fn union2(a: SdfFn, b: SdfFn) -> SdfFn {
	Box::new(move |pos| {
		let a = a(pos);
		let b = b(pos);
		if a.range < b.range { a } else { b }
	})
}

pub fn union3(a: SdfFn, b: SdfFn, c: SdfFn) -> SdfFn {
	union2(a, union2(b, c))
}

pub fn union4(a: SdfFn, b: SdfFn, c: SdfFn, d: SdfFn) -> SdfFn {
	union2(a, union3(b, c, d))
}

pub fn union5(a: SdfFn, b: SdfFn, c: SdfFn, d: SdfFn, e: SdfFn) -> SdfFn {
	union2(a, union4(b, c, d, e))
}

pub fn union6(a: SdfFn, b: SdfFn, c: SdfFn, d: SdfFn, e: SdfFn, f: SdfFn) -> SdfFn {
	union2(a, union5(b, c, d, e, f))
}
