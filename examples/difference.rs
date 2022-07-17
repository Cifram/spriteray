#[path ="common/save_png.rs"]
mod save_png;

use glam::{Vec2, Vec3, Affine3A};
use save_png::save_png;
use spriteray::{Color, render, Difference4, Transform, Sphere};

fn main() {
	let bytes = render(
		&Difference4::new(
			Sphere::new(2.0, Color::new(0.7, 0.7, 0.7)),
			Transform::new(
				Affine3A::from_translation(Vec3::new(0.0, 0.5, -2.0)),
				Sphere::new(1.0, Color::new(1.0, 0.0, 0.0)),
			),
			Transform::new(
				Affine3A::from_translation(Vec3::new(0.5, 0.0, -2.0)),
				Sphere::new(1.0, Color::new(0.0, 1.0, 0.0)),
			),
			Transform::new(
				Affine3A::from_translation(Vec3::new(-0.5, 0.0, -2.0)),
				Sphere::new(1.0, Color::new(0.0, 0.0, 1.0)),
			),
		),
		32, 32, 8.0,
		Vec2::new(4.0, 4.0), Vec3::new(0.0, 2.0, 4.0), Vec3::ZERO,
		Vec3::new(0.3, -1.0, 0.0).normalize()
	);

	save_png("difference", &bytes, 32, 32);
}
