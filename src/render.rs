use glam::{Vec3, Vec2};

use crate::{Color, SdfResult};

const MIN_STEP: f32 = 0.01;

pub type SdfFn = Box<dyn Fn(Vec3) -> SdfResult>;

pub fn render(
	sdf: SdfFn,
	width: usize, height: usize, max_range: f32,
	cam_dims: Vec2, cam_pos: Vec3, cam_target: Vec3,
	light_direction: Vec3
) -> Vec<u8> {
	let start_time = std::time::SystemTime::now();
	let mut pixels = Vec::new();

	let direction = (cam_target - cam_pos).normalize_or_zero();
	let left = direction.cross(Vec3::Y).normalize_or_zero();
	let up = left.cross(direction).normalize_or_zero();
	for y in 0..height {
		for x in 0..width {
			let origin = cam_pos +
				left * cam_dims.x * (x as f32 / (width - 1) as f32 - 0.5) +
				-up * cam_dims.y * (y as f32 / (height - 1) as f32 - 0.5);
			if let Some(color) = raycast(&sdf, origin, direction, max_range, light_direction) {
				pixels.extend_from_slice(&color.bytes());
				pixels.push(255);
			} else {
				pixels.extend_from_slice(&[0, 0, 0, 0]);
			}
		}
	}
	let duration = std::time::SystemTime::now().duration_since(start_time).unwrap();
	println!("Render took {}ms", duration.as_micros() as f64 / 1000.0);

	pixels
}

fn raycast(
	sdf: &SdfFn, ray_start: Vec3, ray_direction: Vec3, ray_length: f32, light_direction: Vec3
) -> Option<Color> {
	let mut len = 0.0;
	while len < ray_length {
		let point = ray_start + ray_direction * len;
		let result = sdf(point);
		if result.range < 0.0 {
			let light_dot = result.normal.dot(-light_direction);
			let ray_dot = result.normal.dot(-ray_direction);
			let color = if light_dot > 0.8 {
				result.color.brighten(0.25)
			} else if light_dot < 0.0 {
				result.color.darken(0.25)
			} else {
				result.color
			};
			let color = if ray_dot < 0.4 {
				color.darken(0.4)
			} else {
				color
			};
			return Some(color);
		} else {
			len += result.range.max(MIN_STEP);
		}
	}
	None
}
