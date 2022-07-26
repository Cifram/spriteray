#[path ="common/save_png.rs"]
mod save_png;

use glam::{Vec2, Vec3, Affine3A};
use save_png::save_png;
use spriteray::{Color, render, transform, sphere, intersection2};

fn main() {
	let bytes = render(
		intersection2(
			transform(
				Affine3A::from_translation(Vec3::X),
				sphere(2.0, Color::new(1.0, 0.0, 0.0)),
			),
			transform(
				Affine3A::from_translation(Vec3::NEG_X),
				sphere(2.0, Color::new(0.0, 1.0, 0.0)),
			),
		),
		32, 32, 8.0,
		Vec2::new(5.0, 5.0), Vec3::new(1.0, 2.0, 4.0), Vec3::ZERO,
		Vec3::new(0.3, -1.0, 0.0).normalize()
	);

	save_png("intersection", &bytes, 32, 32);
}
