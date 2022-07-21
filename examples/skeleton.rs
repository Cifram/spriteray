#[path ="common/save_png.rs"]
mod save_png;

use std::collections::HashMap;

use glam::{Vec2, Vec3, Affine3A, Quat};
use save_png::save_png;
use spriteray::{Color, capsule, sphere, Skeleton, Bone, Pose, Character, skeleton, render};

fn main() {
	let skel = Skeleton::new(HashMap::from([
		("hip".to_string(), Bone::new(
			Affine3A::from_rotation_translation(Quat::IDENTITY, Vec3::Y * -0.5),
			None
		)),
		("left_thigh".to_string(), Bone::new(
			Affine3A::from_rotation_translation(Quat::from_axis_angle(Vec3::X, std::f32::consts::PI), Vec3::X * 0.1),
			Some("hip".to_string()),
		)),
		("left_calf".to_string(), Bone::new(
			Affine3A::from_rotation_translation(Quat::IDENTITY, Vec3::Y * -0.25),
			Some("left_thigh".to_string()),
		)),
		("left_foot".to_string(), Bone::new(
			Affine3A::from_rotation_translation(Quat::from_axis_angle(Vec3::X, std::f32::consts::PI / 2.0), Vec3::Z * -0.25),
			Some("left_calf".to_string()),
		)),
	]));

	let pose = Pose::new(HashMap::from([
		("hip".to_string(), Affine3A::IDENTITY),
		("left_thigh".to_string(), Affine3A::from_axis_angle(Vec3::X, std::f32::consts::PI / 4.0)),
		("left_calf".to_string(), Affine3A::from_axis_angle(Vec3::X, std::f32::consts::PI / -2.0)),
		("left_foot".to_string(), Affine3A::from_axis_angle(Vec3::X, std::f32::consts::PI / 4.0)),
	]));

	let char = Character::new(Vec::from([
		("hip".to_string(), sphere(0.1, Color::new(1.0, 0.0, 0.0))),
		("left_thigh".to_string(), capsule(0.05, 0.03, 0.25, Color::new(0.0, 0.0, 1.0))),
		("left_calf".to_string(), capsule(0.03, 0.02, 0.25, Color::new(0.0, 0.0, 1.0))),
		("left_foot".to_string(), capsule(0.025, 0.01, 0.1, Color::new(1.0, 0.0, 1.0))),
	]));

	let bytes: Vec<u8> = render(
		skeleton(skel, pose, char),
		32, 32, 5.0,
		Vec2::new(0.8, 0.8), Vec3::new(-1.0, 1.0, 2.0), Vec3::new(0.0, 0.5, 0.0),
		Vec3::new(0.3, -1.0, 0.0).normalize()
	);

	save_png("skeleton", &bytes, 32, 32);
}
