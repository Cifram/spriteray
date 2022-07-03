use glam::{Vec2, Vec3};
use spriteray::{Color, render, torus, sphere, union};

fn main() {
	let bytes = render(
		&|point| union(point,
			|point| sphere(point + Vec3::X, 1.5, Color::new(1.0, 0.0, 0.0)),
			|point| sphere(point - Vec3::X, 1.5, Color::new(0.0, 1.0, 0.0)),
		),
		64, 64, 8.0,
		Vec2::new(5.0, 5.0), Vec3::new(0.0, 2.0, 4.0), Vec3::ZERO,
		Vec3::new(0.3, -1.0, 0.0).normalize()
	);

	std::fs::create_dir_all("example_images/").unwrap();
	image::save_buffer("example_images/union.png", &bytes, 64, 64, image::ColorType::Rgba8).unwrap();
}
