#[path ="common/save_png.rs"]
mod save_png;

use glam::{Vec2, Vec3, Quat, Affine3A};
use save_png::save_png;
use spriteray::{Color, capsule, Character, skeleton, anim_render, build_humanoid_skeleton, HumanoidProportions, build_humanoid_pose, HumanoidPoseDescriptor, half_capsule, difference2, transform, union2, rounded_quad, plane_normal_offset, sphere};

const RESOLUTION: usize = 40;
const FRAMES: usize = 8;

const PI: f32 = std::f32::consts::PI;
const SKIN: Color = Color::new(0.92, 0.74, 0.62);

fn main() {
	let props = HumanoidProportions::chibi();
	let skel = build_humanoid_skeleton(props);

	let bytes: Vec<u8> = anim_render(
		Box::new(move |time| {
			let leg_length = props.thigh_length + props.calf_length;
			let (left_foot_position, left_toes_vertical_offset) = walk_foot_position_by_time((time + 0.5) % 1.0, 0.18);
			let (right_foot_position, right_toes_vertical_offset) = walk_foot_position_by_time(time, -0.18);
			let walk_period_1 = (time * 2.0 * PI).sin();
			let walk_period_2 = (time * 2.0 * PI).cos();
			let pose = build_humanoid_pose(props, HumanoidPoseDescriptor {
				hip_height: leg_length - 0.05 + walk_period_1.abs() * 0.1,
				hip_rotation: Quat::from_rotation_y((time * 2.0 * PI).cos() * PI / 16.0),
				chest_rotation: Quat::from_rotation_y((time * 2.0 * PI).cos() * -PI / 8.0),
				head_rotation: Quat::IDENTITY,
				left_foot_position,
				left_toes_vertical_offset,
				right_foot_position,
				right_toes_vertical_offset,
				left_hand_position: Vec3::new(props.shoulder_width, leg_length + 0.1 + walk_period_1.abs() * 0.0, -walk_period_2 * 0.3),
				left_hand_rotation: Quat::IDENTITY,
				right_hand_position: Vec3::new(-props.shoulder_width, leg_length + 0.1 + walk_period_1.abs() * 0.0, walk_period_2 * 0.3),
				right_hand_rotation: Quat::IDENTITY,
			}, &skel);
			transform(
				Affine3A::from_rotation_y(-PI/16.0 + PI/2.0),
				skeleton(skel.clone(), pose, make_char(props)),
			)
		}),
		RESOLUTION, RESOLUTION, 5.0,
		0.0, 1.0, 1.0 / FRAMES as f32,
		Vec2::new(2.3, 2.3), Vec3::new(0.0, 1.2, 2.0), Vec3::new(0.0, 1.1, 0.0),
		Vec3::new(0.0, -1.0, -0.5).normalize()
	).into_iter().flatten().collect();

	save_png("humanoid", &bytes, RESOLUTION as u32, (RESOLUTION * FRAMES) as u32);
}

fn make_char(props: HumanoidProportions) -> Character {
	let thigh_width = props.hip_width * 1.2;
	let knee_width = thigh_width * 0.75;
	let ankle_width = thigh_width * 0.5;
	let foot_width = ankle_width * 1.5;
	let shoulder_width = thigh_width * 0.75;
	let elbow_width = shoulder_width;
	let wrist_width = elbow_width;
	let hand_width = wrist_width * 1.2;
	let torso_width = thigh_width * 1.5;
	let head_radius = torso_width * 1.3;
	let midriff_height = props.torso_height * 0.66;
	let chest_height = props.torso_height * 0.34;
	let neck_width = thigh_width;

	Character::new(Vec::from([
		(
			"hips".to_string(),
			transform(
				Affine3A::from_rotation_translation(Quat::from_rotation_z(-PI/2.0), Vec3::NEG_X * props.hip_width),
				capsule(thigh_width, thigh_width, props.hip_width * 2.0, SKIN),
			)
		),
		(
			"left_thigh".to_string(),
			capsule(thigh_width, knee_width, props.thigh_length, SKIN)
		),
		(
			"left_calf".to_string(),
			half_capsule(knee_width, ankle_width, props.calf_length - ankle_width, SKIN)
		),
		(
			"left_foot".to_string(),
			difference2(
				capsule(foot_width, foot_width, props.foot_length, SKIN),
				plane_normal_offset(Vec3::NEG_Z, 0.0, SKIN),
			)
		),
		(
			"right_thigh".to_string(),
			capsule(thigh_width, knee_width, props.thigh_length, SKIN)
		),
		(
			"right_calf".to_string(),
			half_capsule(knee_width, ankle_width, props.calf_length - ankle_width, SKIN)
		),
		(
			"right_foot".to_string(),
			difference2(
				capsule(foot_width, foot_width, props.foot_length, SKIN),
				plane_normal_offset(Vec3::NEG_Z, 0.0, SKIN),
			)
		),
		(
			"midriff".to_string(),
			rounded_quad(torso_width, torso_width, midriff_height, thigh_width, thigh_width, SKIN),
		),
		(
			"chest".to_string(),
			union2(
				rounded_quad(torso_width, torso_width, chest_height - (thigh_width - shoulder_width), thigh_width, thigh_width, SKIN),
				transform(
					Affine3A::from_rotation_translation(
						Quat::from_rotation_z(-PI / 2.0),
						Vec3::new(-props.shoulder_width, chest_height, 0.0),
					),
					capsule(shoulder_width, shoulder_width, props.shoulder_width * 2.0, SKIN),
				),
			),
		),
		(
			"neck".to_string(),
			capsule(neck_width, neck_width * 0.8, props.neck_length, SKIN),
		),
		(
			"head".to_string(),
			transform(
				Affine3A::from_translation(Vec3::new(0.0, head_radius, 0.0)),
				transform(
					Affine3A::from_rotation_x(3.0 * PI / 4.0),
					capsule(head_radius, head_radius / 2.0, head_radius * 0.75, SKIN),
				),
			),
		),
		(
			"left_upper_arm".to_string(),
			capsule(shoulder_width, elbow_width, props.upper_arm_length, SKIN),
		),
		(
			"left_lower_arm".to_string(),
			capsule(elbow_width, wrist_width, props.lower_arm_length, SKIN),
		),
		(
			"left_hand".to_string(),
			transform(
				Affine3A::from_translation(Vec3::Y * hand_width / 2.0),
				sphere(hand_width, SKIN),
			),
		),
		(
			"right_upper_arm".to_string(),
			capsule(shoulder_width, elbow_width, props.upper_arm_length, SKIN),
		),
		(
			"right_lower_arm".to_string(),
			capsule(elbow_width, wrist_width, props.lower_arm_length, SKIN),
		),
		(
			"right_hand".to_string(),
			transform(
				Affine3A::from_translation(Vec3::Y * hand_width / 2.0),
				sphere(hand_width, SKIN),
			),
		),
	]))
}

fn walk_foot_position_by_time(time: f32, x: f32) -> (Vec3, f32) {
	if time < 0.5 {
		(
			Vec3::new(x, (time * 2.0 * PI).sin() * 0.1 + (0.5 - time) * 2.0 * 0.1, time - 0.25),
			(0.5 - time) * 2.0 * -0.1,
		)
	} else {
		(
			Vec3::new(x, (time - 0.5) * 2.0 * 0.1, 0.75 - time),
			(time - 0.5) * 2.0 * -0.1,
		)
	}
}
