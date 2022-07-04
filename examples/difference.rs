use glam::{Vec2, Vec3};
use spriteray::{Color, sphere, render, SdfResult, difference4};

fn main() {
	let bytes = render(
		&sdf,
		64, 64, 8.0,
		Vec2::new(4.0, 4.0), Vec3::new(0.0, 2.0, 4.0), Vec3::ZERO,
		Vec3::new(0.3, -1.0, 0.0).normalize()
	);

	std::fs::create_dir_all("example_images/").unwrap();
	image::save_buffer("example_images/difference.png", &bytes, 64, 64, image::ColorType::Rgba8).unwrap();
}

fn sdf(point: Vec3) -> SdfResult {
	difference4(point,
		&|point| sphere(point, 2.0, Color::new(0.7, 0.7, 0.7)),
		&|point| sphere(point - Vec3::Z*2.0 - Vec3::Y*0.5, 1.0, Color::new(1.0, 0.0, 0.0)),
		&|point| sphere(point - Vec3::Z*2.0 + Vec3::X*0.5, 1.0, Color::new(0.0, 1.0, 0.0)),
		&|point| sphere(point - Vec3::Z*2.0 - Vec3::X*0.5, 1.0, Color::new(0.0, 0.0, 1.0)),
	)
}
