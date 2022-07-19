#[path ="common/save_png.rs"]
mod save_png;

use glam::{Vec2, Vec3};
use save_png::save_png;
use spriteray::{Color, render, cone};

fn main() {
	let bytes = render(
		cone(1.0, 2.0, Color::new(1.0, 0.0, 0.0)),
		32, 32, 16.0,
		Vec2::new(3.0, 3.0), Vec3::new(0.0, 2.0, 4.0), Vec3::new(0.0, 1.0, 0.0),
		Vec3::new(1.0, -0.3, 0.0).normalize()
	);

	save_png("cone", &bytes, 32, 32);
}
