#[path ="common/save_png.rs"]
mod save_png;

use glam::{Vec2, Vec3};
use save_png::save_png;
use spriteray::{Color, render, HalfCapsule};

fn main() {
	let bytes = render(
		&HalfCapsule::new(2.0, 1.0, 4.0, Color::new(1.0, 0.0, 0.0)),
		32, 32, 5.0,
		Vec2::new(7.0, 7.0), Vec3::new(0.0, 4.0, 4.0), Vec3::new(0.0, 2.0, 0.0),
		Vec3::new(0.3, -1.0, 0.0).normalize()
	);

	save_png("half_capsule", &bytes, 32, 32);
}
