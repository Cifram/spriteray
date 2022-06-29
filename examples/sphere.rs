use glam::{Vec2, Vec3};
use spriteray::{Color, render, sphere};

fn main() {
	let bytes = render(
		&|point| sphere(point, Vec3::ZERO, 1.0, Color::new(1.0, 0.0, 0.0)),
		32, 32, 5.0,
		Vec2::new(2.0, 2.0), Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO
	);

	image::save_buffer("sphere.png", &bytes, 32, 32, image::ColorType::Rgba8).unwrap();
}
