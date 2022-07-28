use glam::{Vec3, Vec2};

use crate::{Color, SdfResult};

const MIN_STEP: f32 = 0.01;

pub type SdfFn = Box<dyn Fn(Vec3) -> SdfResult>;

#[derive(Copy, Clone, PartialEq)]
enum Pixel {
	Empty,
	NonEmpty {
		color: Color,
		normal: Vec3,
		depth: f32,
	}
}

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
		pixels.push(Vec::new());
		for x in 0..width {
			let origin = cam_pos +
				left * cam_dims.x * (x as f32 / (width - 1) as f32 - 0.5) +
				-up * cam_dims.y * (y as f32 / (height - 1) as f32 - 0.5);
			pixels[y].push(raycast(&sdf, origin, direction, max_range));
		}
	}
	let duration = std::time::SystemTime::now().duration_since(start_time).unwrap();
	println!("Render took {}ms", duration.as_micros() as f64 / 1000.0);

	let mut image = Vec::new();
	for y in 0..height {
		for x in 0..width {
			if let Pixel::NonEmpty { color, normal: _, depth: _ } = pixels[y][x] {
				let light_value = calc_light(x, y, &pixels, -light_direction);
				let halo_value = calc_halo(x, y, &pixels, -direction);
				let outline_value = calc_outline(x, y, &pixels);
				let depth_value = calc_depth(x, y, &pixels, 2.0, 2.5);
				let ssao_value = calc_ssao(x, y, &pixels);
				let brightness = light_value * halo_value * outline_value * depth_value * ssao_value;
				let brightness = (brightness * 4.0).round() / 4.0;
				image.extend_from_slice(&(color * brightness).bytes());
				image.push(255);
			} else {
				image.extend_from_slice(&[0, 0, 0, 0]);
			}
		}
	}

	image
}

fn calc_light(x: usize, y: usize, pixels: &Vec<Vec<Pixel>>, light_direction: Vec3) -> f32 {
	if let Pixel::NonEmpty { color: _, normal, depth: _ } = pixels[y][x] {
		let light = light_direction.dot(normal);
		let light = 1.0 - light;
		let light = light * light;
		let light = light * light;
		(1.0 - light).clamp(0.5, 1.0)
	} else {
		0.0
	}
}

fn calc_halo(x: usize, y: usize, pixels: &Vec<Vec<Pixel>>, camera_direction: Vec3) -> f32 {
	if let Pixel::NonEmpty { color: _, normal, depth: _ } = pixels[y][x] {
		let halo = camera_direction.dot(normal);
		let halo = 1.0 - halo;
		let halo = halo * halo;
		1.0 - halo
	} else {
		0.0
	}
}

fn calc_outline(x: usize, y: usize, pixels: &Vec<Vec<Pixel>>) -> f32 {
	if let Pixel::Empty = pixels[y][x] {
		0.25
	} else {
		let edge =
			(get_offset_pixel(pixels, x, y, 0, -1) == Pixel::Empty) ||
			(get_offset_pixel(pixels, x, y, 0, 1) == Pixel::Empty) ||
			(get_offset_pixel(pixels, x, y, -1, 0) == Pixel::Empty) ||
			(get_offset_pixel(pixels, x, y, 1, 0) == Pixel::Empty);
		if edge { 0.5 } else { 1.0 }
	}
}

fn calc_depth(x: usize, y: usize, pixels: &Vec<Vec<Pixel>>, min_depth: f32, max_depth: f32) -> f32 {
	if let Pixel::NonEmpty { color: _, normal: _, depth } = pixels[y][x] {
		1.0 - ((depth - min_depth) / (max_depth - min_depth)).clamp(0.0, 1.0)
	} else {
		0.0
	}
}

fn calc_ssao(x: usize, y: usize, pixels: &Vec<Vec<Pixel>>) -> f32 {
	if let Pixel::NonEmpty { color: _, normal: _, depth } = pixels[y][x] {
		let mut darkening = 0.0;
		let mut test_pixel = |dx, dy| {
			if let Pixel::NonEmpty { color: _, normal: _, depth: neighor_depth } = get_offset_pixel(pixels, x, y, dx, dy) {
				if depth < neighor_depth {
					darkening += (neighor_depth - depth) / ((dx*dx + dy*dy) as f32).sqrt();
				}
			}
		};
		for dx in -2..2 {
			for dy in -2..2 {
				if dx == 0 && dy == 0 {
					continue;
				}
				test_pixel(dx, dy);
			}
		}
		(1.5 * 4.0_f32.powf(-darkening)).clamp(0.0, 1.0)
	} else {
		0.0
	}
}

fn get_offset_pixel(pixels: &Vec<Vec<Pixel>>, x: usize, y: usize, dx: isize, dy: isize) -> Pixel {
	let x = x as isize - dx;
	let y = y as isize - dy;
	let width = pixels[0].len() as isize;
	let height = pixels.len() as isize;
	if x < 0 || x > width-1 || y < 0 || y > height-1 {
		Pixel::Empty
	} else {
		pixels[y as usize][x as usize]
	}
}

fn raycast(
	sdf: &SdfFn, ray_start: Vec3, ray_direction: Vec3, ray_length: f32
) -> Pixel {
	let mut len = 0.0;
	while len < ray_length {
		let point = ray_start + ray_direction * len;
		let result = sdf(point);
		if result.range < 0.0 {
			return Pixel::NonEmpty {
				color: result.color,
				normal: result.normal,
				depth: len,
			};
		} else {
			len += result.range.max(MIN_STEP);
		}
	}
	Pixel::Empty
}
