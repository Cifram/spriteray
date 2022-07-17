#[path ="common/save_png.rs"]
mod save_png;

use glam::{Vec2, Vec3};
use save_png::save_png;
use spriteray::{Color, render, Sphere};

fn main() {
	let bytes = render(
		&Sphere::new(1.0, Color::new(1.0, 0.0, 0.0)),
		32, 32, 5.0,
		Vec2::new(2.0, 2.0), Vec3::new(0.0, 2.0, 4.0), Vec3::ZERO,
		Vec3::new(0.3, -1.0, 0.0).normalize()
	);

	save_png("sphere", &bytes, 32, 32);
}
