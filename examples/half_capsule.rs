use glam::{Vec2, Vec3};
use spriteray::{Color, render, HalfCapsule};

fn main() {
	let bytes = render(
		&HalfCapsule::new(2.0, 1.0, 4.0, Color::new(1.0, 0.0, 0.0)),
		54, 54, 5.0,
		Vec2::new(7.0, 7.0), Vec3::new(0.0, 4.0, 4.0), Vec3::new(0.0, 2.0, 0.0),
		Vec3::new(0.3, -1.0, 0.0).normalize()
	);

	std::fs::create_dir_all("example_images/").unwrap();
	image::save_buffer("example_images/half_capsule.png", &bytes, 54, 54, image::ColorType::Rgba8).unwrap();
}
