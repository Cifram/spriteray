#[path ="common/save_png.rs"]
mod save_png;

use glam::{Vec2, Vec3};
use save_png::save_png;
use spriteray::{Color, render, rounded_quad};

fn main() {
	let bytes = render(
		rounded_quad(2.0, 4.0, 3.0, 1.0, 0.5, Color::new(1.0, 0.0, 0.0)),
		128, 128, 5.0,
		Vec2::new(6.0, 6.0), Vec3::new(1.0, 1.5, 2.0), Vec3::new(0.0, 1.0, 0.0),
		Vec3::new(0.3, -1.0, 0.0).normalize()
	);

	save_png("rounded_quad", &bytes, 128, 128);
}
