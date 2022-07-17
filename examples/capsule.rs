#[path ="common/save_png.rs"]
mod save_png;

use glam::{Vec2, Vec3};
use save_png::save_png;
use spriteray::{Color, render, Capsule};

fn main() {
	let bytes = render(
		&Capsule::new(1.5, 1.0, 4.0, Color::new(1.0, 0.0, 0.0)),
		32, 32, 5.0,
		Vec2::new(7.0, 7.0), Vec3::new(0.0, 4.0, 4.0), Vec3::new(0.0, 2.0, 0.0),
		Vec3::new(0.3, -1.0, 0.0).normalize()
	);

	save_png("capsule", &bytes, 32, 32);
}
