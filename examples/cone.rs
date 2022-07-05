use glam::{Vec2, Vec3};
use spriteray::{Color, render, Cone};

fn main() {
	let bytes = render(
		&Cone::new(1.0, 2.0, Color::new(1.0, 0.0, 0.0)),
		64, 64, 16.0,
		Vec2::new(3.0, 3.0), Vec3::new(0.0, 2.0, 4.0), Vec3::new(0.0, 1.0, 0.0),
		Vec3::new(1.0, -0.3, 0.0).normalize()
	);

	std::fs::create_dir_all("example_images/").unwrap();
	image::save_buffer("example_images/cone.png", &bytes, 64, 64, image::ColorType::Rgba8).unwrap();
}
