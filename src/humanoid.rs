use std::{collections::HashMap};

use glam::{Quat, Vec3, Affine3A};

use crate::{Skeleton, Bone, Pose};

#[derive(Copy, Clone)]
pub struct HumanoidProportions {
	pub upper_arm_length: f32,
	pub lower_arm_length: f32,
	pub thigh_length: f32,
	pub calf_length: f32,
	pub foot_length: f32,
	pub torso_height: f32,
	pub hip_width: f32,
	pub shoulder_width: f32,
	pub neck_length: f32,
}

impl HumanoidProportions {
	pub fn chibi() -> Self {
		Self {
			upper_arm_length: 0.3,
			lower_arm_length: 0.3,
			thigh_length: 0.3,
			calf_length: 0.3,
			foot_length: 0.15,
			torso_height: 0.4,
			hip_width: 0.18,
			shoulder_width: 0.2,
			neck_length: 0.0,
		}
	}
}

pub struct HumanoidPoseDescriptor {
	pub hip_height: f32,
	pub torso_rotation: Quat,
	pub head_rotation: Quat,
	pub left_foot_position: Vec3,
	pub left_toes_vertical_offset: f32,
	pub right_foot_position: Vec3,
	pub right_toes_vertical_offset: f32,
	pub left_hand_position: Vec3,
	pub left_hand_rotation: Quat,
	pub right_hand_position: Vec3,
	pub right_hand_rotation: Quat,
}

const PI: f32 = std::f32::consts::PI;

pub fn build_humanoid_skeleton(props: HumanoidProportions) -> Skeleton {
	Skeleton::new(HashMap::from([
		(
			"hips".to_string(),
			Bone::new(
				Affine3A::from_translation(Vec3::NEG_Y * (props.thigh_length + props.calf_length)),
				None,
			),
		),
		(
			"left_thigh".to_string(),
			Bone::new(
				Affine3A::from_rotation_translation(Quat::from_rotation_x(PI), Vec3::X * props.hip_width),
				Some("hips".to_string()),
			),
		),
		(
			"left_calf".to_string(),
			Bone::new(
				Affine3A::from_translation(Vec3::NEG_Y * props.thigh_length),
				Some("left_thigh".to_string()),
			),
		),
		(
			"left_foot".to_string(),
			Bone::new(
				Affine3A::from_rotation_translation(Quat::from_rotation_x(PI / 2.0) * Quat::from_rotation_y(PI / -8.0), Vec3::NEG_Z * props.calf_length),
				Some("left_calf".to_string()),
			),
		),
		(
			"right_thigh".to_string(),
			Bone::new(
				Affine3A::from_rotation_translation(Quat::from_rotation_x(PI), Vec3::NEG_X * props.hip_width),
				Some("hips".to_string()),
			),
		),
		(
			"right_calf".to_string(),
			Bone::new(
				Affine3A::from_translation(Vec3::NEG_Y * props.thigh_length),
				Some("right_thigh".to_string()),
			),
		),
		(
			"right_foot".to_string(),
			Bone::new(
				Affine3A::from_rotation_translation(Quat::from_rotation_x(PI / 2.0) * Quat::from_rotation_y(PI / 8.0), Vec3::NEG_Z * props.calf_length),
				Some("right_calf".to_string()),
			),
		),
	]))
}

pub fn build_humanoid_pose(props: HumanoidProportions, pose: HumanoidPoseDescriptor) -> Pose {
	let base_hip_height = props.thigh_length + props.calf_length;
	let hip_pos = Vec3::NEG_Y * pose.hip_height;

	let (left_leg_rotation, left_thigh_angle, left_knee_angle, left_foot_rotation) = leg_ik(
		props,
		pose.torso_rotation * Vec3::X,
		base_hip_height,
		hip_pos,
		pose.left_foot_position,
		pose.left_toes_vertical_offset
	);
	let (right_leg_rotation, right_thigh_angle, right_knee_angle, right_foot_rotation) = leg_ik(
		props,
		pose.torso_rotation * Vec3::NEG_X,
		base_hip_height,
		hip_pos,
		pose.right_foot_position,
		pose.right_toes_vertical_offset
	);

	Pose::new(HashMap::from([
		(
			"hips".to_string(),
			Affine3A::from_translation(Vec3::Y * (base_hip_height - pose.hip_height)),
		),
		(
			"left_thigh".to_string(),
			Affine3A::from_rotation_x(left_thigh_angle) * Affine3A::from_rotation_translation(left_leg_rotation, Vec3::ZERO),
		),
		(
			"left_calf".to_string(),
			Affine3A::from_rotation_x(left_knee_angle),
		),
		(
			"left_foot".to_string(),
			Affine3A::from_rotation_translation(left_foot_rotation, Vec3::ZERO),
		),
		(
			"right_thigh".to_string(),
			Affine3A::from_rotation_x(right_thigh_angle) * Affine3A::from_rotation_translation(right_leg_rotation, Vec3::ZERO),
		),
		(
			"right_calf".to_string(),
			Affine3A::from_rotation_x(right_knee_angle),
		),
		(
			"right_foot".to_string(),
			Affine3A::from_rotation_translation(right_foot_rotation, Vec3::ZERO),
		),
	]))
}

fn leg_ik(
	props: HumanoidProportions,
	hip_dir: Vec3,
	base_hip_height: f32,
	hip_pos: Vec3,
	foot_position: Vec3,
	toe_vertical_offset: f32,
) -> (Quat, f32, f32, Quat) {
	assert!(props.foot_length.abs() > toe_vertical_offset.abs(),
		"The toe_vertical_offset of {} is longer than the foot length of {}", toe_vertical_offset, props.foot_length
	);

	let thigh_length_sqr = props.thigh_length * props.thigh_length;
	let calf_length_sqr = props.calf_length * props.calf_length;

	let hip_pos = hip_dir * props.hip_width + hip_pos;
	let foot_offset = foot_position - hip_pos;
	let mut foot_range_sqr = foot_offset.length_squared();
	let mut foot_range = foot_range_sqr.sqrt();
	if foot_range > base_hip_height {
		foot_range = base_hip_height;
		foot_range_sqr = foot_range * foot_range;
	}
	let foot_dir = foot_offset / foot_range;
	let leg_rotation = Quat::from_rotation_arc(Vec3::Y, foot_dir);
	let thigh_angle = (
		(foot_range_sqr + thigh_length_sqr - calf_length_sqr) /
		(2.0 * foot_range * props.thigh_length)
	).acos();
	let knee_angle = (
		(thigh_length_sqr + calf_length_sqr - foot_range_sqr) /
		(2.0 * props.thigh_length * props.calf_length)
	).acos() - PI;
	let foot_rotation =
		leg_rotation.inverse() *
		Quat::from_rotation_x(-thigh_angle) *
		Quat::from_rotation_x(-knee_angle) *
		Quat::from_rotation_arc(Vec3::Y,
			(
				Vec3::Y * (props.foot_length * props.foot_length - toe_vertical_offset * toe_vertical_offset).sqrt() +
				Vec3::Z * toe_vertical_offset
			).normalize()
		);

	(leg_rotation, thigh_angle, knee_angle, foot_rotation)
}
