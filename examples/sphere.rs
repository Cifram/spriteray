use glam::{Vec2, Vec3};
use spriteray::{Color, render, Sphere};

fn main() {
	let bytes = render(
		&Sphere::new(1.0, Color::new(1.0, 0.0, 0.0)),
		32, 32, 5.0,
		Vec2::new(2.0, 2.0), Vec3::new(0.0, 2.0, 4.0), Vec3::ZERO,
		Vec3::new(0.3, -1.0, 0.0).normalize()
	);

	std::fs::create_dir_all("example_images/").unwrap();
	image::save_buffer("example_images/sphere.png", &bytes, 32, 32, image::ColorType::Rgba8).unwrap();
}
