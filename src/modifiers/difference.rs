use crate::{SdfResult, SdfFn, union2, union3, union4, union5};

pub fn difference2(a: SdfFn, b: SdfFn) -> SdfFn {
	Box::new(move |pos| {
		let a = a(pos);
		let b = b(pos);
		if a.range > -b.range {
			a
		} else {
			SdfResult {
				range: -b.range,
				normal: -b.normal,
				color: b.color,
			}
		}
	})
}

pub fn difference3(a: SdfFn, b: SdfFn, c: SdfFn) -> SdfFn {
	difference2(a, union2(b, c))
}

pub fn difference4(a: SdfFn, b: SdfFn, c: SdfFn, d: SdfFn) -> SdfFn {
	difference2(a, union3(b, c, d))
}

pub fn difference5(a: SdfFn, b: SdfFn, c: SdfFn, d: SdfFn, e: SdfFn) -> SdfFn {
	difference2(a, union4(b, c, d, e))
}

pub fn difference6(a: SdfFn, b: SdfFn, c: SdfFn, d: SdfFn, e: SdfFn, f: SdfFn) -> SdfFn {
	difference2(a, union5(b, c, d, e, f))
}
