use glam::{Vec2, Vec3};

use crate::{render, SdfFn};

pub fn anim_render(
	sdf_fn: Box<dyn Fn(f32) -> SdfFn>,
	width: usize, height: usize, max_range: f32,
	start_time: f32, end_time: f32, frame_time: f32,
	cam_dims: Vec2, cam_pos: Vec3, cam_target: Vec3,
	light_direction: Vec3
) -> Vec<Vec<u8>> {
	let time_range = end_time - start_time;
	let frame_count = (time_range / frame_time) as isize;
	let mut frames = Vec::new();
	for frame in 0..frame_count {
		println!("Frame {}", frame);
		let sdf = sdf_fn(start_time + frame as f32 * frame_time);
		frames.push(render(sdf, width, height, max_range, cam_dims, cam_pos, cam_target, light_direction));
	}
	frames
}
