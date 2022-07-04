use glam::{Vec2, Vec3};
use spriteray::{Color, render, SdfResult, capsule};

fn main() {
	let bytes = render(
		&sdf,
		64, 64, 5.0,
		Vec2::new(7.0, 7.0), Vec3::new(0.0, 4.0, 4.0), Vec3::new(0.0, 2.0, 0.0),
		Vec3::new(0.3, -1.0, 0.0).normalize()
	);

	std::fs::create_dir_all("example_images/").unwrap();
	image::save_buffer("example_images/capsule.png", &bytes, 64, 64, image::ColorType::Rgba8).unwrap();
}

fn sdf(point: Vec3) -> SdfResult {
	capsule(point, 1.5, 1.0, 4.0, Color::new(1.0, 0.0, 0.0))
}
