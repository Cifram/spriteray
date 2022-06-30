use glam::{Vec2, Vec3};
use spriteray::{Color, render, sphere};

fn main() {
	let bytes = render(
		&|point| sphere(point, 1.0, Color::new(1.0, 0.0, 0.0)),
		32, 32, 5.0,
		Vec2::new(2.0, 2.0), Vec3::new(0.0, 2.0, 4.0), Vec3::ZERO,
		Vec3::new(0.3, -1.0, 0.0).normalize()
	);

	image::save_buffer("sphere.png", &bytes, 32, 32, image::ColorType::Rgba8).unwrap();
}
