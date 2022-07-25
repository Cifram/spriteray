#[path ="common/save_png.rs"]
mod save_png;

use glam::{Vec2, Vec3, Quat, Affine3A};
use save_png::save_png;
use spriteray::{Color, capsule, Character, skeleton, anim_render, build_humanoid_skeleton, HumanoidProportions, build_humanoid_pose, HumanoidPoseDescriptor, half_capsule, difference2, plane, transform};

const SKIN: Color = Color::new(0.84, 0.74, 0.62);

fn main() {
	let props = HumanoidProportions::chibi();
	let skel = build_humanoid_skeleton(props);

	let bytes: Vec<u8> = anim_render(
		Box::new(move |time| {
			let (left_foot_position, left_toes_vertical_offset) = walk_foot_position_by_time(time, 0.18);
			let (right_foot_position, right_toes_vertical_offset) = walk_foot_position_by_time((time + 0.5) % 1.0, -0.18);
			let pose = build_humanoid_pose(props, HumanoidPoseDescriptor {
				hip_height: 0.5 + ((time * 2.0 * std::f32::consts::PI).sin() * 0.15).abs(),
				torso_rotation: Quat::IDENTITY,
				head_rotation: Quat::IDENTITY,
				left_foot_position,
				left_toes_vertical_offset,
				right_foot_position,
				right_toes_vertical_offset,
				left_hand_position: Vec3::new(0.6, 0.7, 0.0),
				left_hand_rotation: Quat::IDENTITY,
				right_hand_position: Vec3::new(-0.6, 0.7, 0.0),
				right_hand_rotation: Quat::IDENTITY,
			});
			skeleton(skel.clone(), pose, make_char())
		}),
		40, 40, 5.0,
		0.0, 1.0, 1.0 / 8.0,
		Vec2::new(2.1, 2.1), Vec3::new(2.0, 2.0, 0.25), Vec3::new(0.0, 1.0, 0.0),
		Vec3::new(0.3, -1.0, 0.0).normalize()
	).into_iter().flatten().collect();

	save_png("humanoid", &bytes, 40, 40 * 8);
}

fn make_char() -> Character {
	Character::new(Vec::from([
		(
			"hips".to_string(),
			transform(
				Affine3A::from_rotation_translation(Quat::from_rotation_z(std::f32::consts::PI/2.0), Vec3::Y * 0.18),
				capsule(0.2, 0.2, 0.36, SKIN),
			)
		),
		(
			"left_thigh".to_string(),
			capsule(0.2, 0.15, 0.3, SKIN)
		),
		(
			"left_calf".to_string(),
			half_capsule(0.15, 0.1, 0.2, SKIN)
		),
		(
			"left_foot".to_string(),
			difference2(
				capsule(0.12, 0.1, 0.15, SKIN),
				transform(
					Affine3A::from_rotation_x(std::f32::consts::PI / 2.0),
					plane(SKIN),
				)
			)
		),
		(
			"right_thigh".to_string(),
			capsule(0.2, 0.15, 0.3, SKIN)
		),
		(
			"right_calf".to_string(),
			half_capsule(0.15, 0.1, 0.2, SKIN)
		),
		(
			"right_foot".to_string(),
			difference2(
				capsule(0.12, 0.1, 0.15, SKIN),
				transform(
					Affine3A::from_rotation_x(std::f32::consts::PI / 2.0),
					plane(SKIN),
				)
			)
		),
	]))
}

fn walk_foot_position_by_time(time: f32, x: f32) -> (Vec3, f32) {
	if time < 0.5 {
		(
			Vec3::new(x, (time * 2.0 * std::f32::consts::PI).sin() * -0.2, time - 0.25),
			(0.5 - time) * 2.0 * -0.05,
		)
	} else {
		(
			Vec3::new(x, (time - 0.5) * 2.0 * -0.05, 0.75 - time),
			(time - 0.5) * 2.0 * -0.05,
		)
	}
}
