use glam::{Vec2, Vec3, Affine3A};
use spriteray::{Color, Sphere, Union3, anim_render, Transform};

fn main() {
	let bytes: Vec<u8> = anim_render(
		|time| {
			let time1 = time;
			let time2 = time + std::f32::consts::PI * 2.0 / 3.0;
			let time3 = time + std::f32::consts::PI * 4.0 / 3.0;
			return Union3::new(
				Transform::new(
					Affine3A::from_translation(Vec3::new(time1.sin(), time1.cos(), 0.0)),
					Sphere::new(1.0, Color::new(1.0, 0.0, 0.0)),
				),
				Transform::new(
					Affine3A::from_translation(Vec3::new(time2.sin(), time2.cos(), 0.0)),
					Sphere::new(1.0, Color::new(0.0, 1.0, 0.0)),
				),
				Transform::new(
					Affine3A::from_translation(Vec3::new(time3.sin(), time3.cos(), 0.0)),
					Sphere::new(1.0, Color::new(0.0, 0.0, 1.0)),
				),
			)
		},
		32, 32, 5.0,
		0.0, std::f32::consts::PI * 2.0, std::f32::consts::PI / 4.0,
		Vec2::new(4.0, 4.0), Vec3::new(0.0, 2.0, 4.0), Vec3::ZERO,
		Vec3::new(0.3, -1.0, 0.0).normalize()
	).into_iter().flatten().collect();

	std::fs::create_dir_all("example_images/").unwrap();
	image::save_buffer("example_images/animation.png", &bytes, 32, 32*8, image::ColorType::Rgba8).unwrap();
}
