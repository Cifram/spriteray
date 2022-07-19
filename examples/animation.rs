#[path ="common/save_png.rs"]
mod save_png;

use glam::{Vec2, Vec3, Affine3A};
use save_png::save_png;
use spriteray::{Color, anim_render, union3, transform, sphere};

fn main() {
	let bytes: Vec<u8> = anim_render(
		|time| {
			let time1 = time;
			let time2 = time + std::f32::consts::PI * 2.0 / 3.0;
			let time3 = time + std::f32::consts::PI * 4.0 / 3.0;
			union3(
				transform(
					Affine3A::from_translation(Vec3::new(time1.sin(), time1.cos(), 0.0)),
					sphere(1.0, Color::new(1.0, 0.0, 0.0)),
				),
				transform(
					Affine3A::from_translation(Vec3::new(time2.sin(), time2.cos(), 0.0)),
					sphere(1.0, Color::new(0.0, 1.0, 0.0)),
				),
				transform(
					Affine3A::from_translation(Vec3::new(time3.sin(), time3.cos(), 0.0)),
					sphere(1.0, Color::new(0.0, 0.0, 1.0)),
				)
			)
		},
		32, 32, 5.0,
		0.0, std::f32::consts::PI * 2.0, std::f32::consts::PI / 4.0,
		Vec2::new(4.0, 4.0), Vec3::new(0.0, 2.0, 4.0), Vec3::ZERO,
		Vec3::new(0.3, -1.0, 0.0).normalize()
	).into_iter().flatten().collect();

	save_png("animation", &bytes, 32, 32*8);
}
