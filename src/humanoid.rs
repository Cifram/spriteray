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
			upper_arm_length: 0.25,
			lower_arm_length: 0.25,
			thigh_length: 0.35,
			calf_length: 0.35,
			foot_length: 0.15,
			torso_height: 0.5,
			hip_width: 0.2,
			shoulder_width: 0.4,
			neck_length: 0.05,
		}
	}
}

pub struct HumanoidPoseDescriptor {
	pub hip_height: f32,
	pub hip_rotation: Quat,
	pub chest_rotation: Quat,
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
				Affine3A::from_translation(Vec3::Y * (props.thigh_length + props.calf_length)),
				None,
			),
		),
		(
			"left_thigh".to_string(),
			Bone::new(
				Affine3A::from_rotation_translation(Quat::from_rotation_x(-PI), Vec3::X * props.hip_width),
				Some("hips".to_string()),
			),
		),
		(
			"left_calf".to_string(),
			Bone::new(
				Affine3A::from_translation(Vec3::Y * props.thigh_length),
				Some("left_thigh".to_string()),
			),
		),
		(
			"left_foot".to_string(),
			Bone::new(
				Affine3A::from_rotation_translation(Quat::from_rotation_x(-PI / 2.0) * Quat::from_rotation_z(PI / -8.0), Vec3::Y * props.calf_length),
				Some("left_calf".to_string()),
			),
		),
		(
			"right_thigh".to_string(),
			Bone::new(
				Affine3A::from_rotation_translation(Quat::from_rotation_x(-PI), Vec3::NEG_X * props.hip_width),
				Some("hips".to_string()),
			),
		),
		(
			"right_calf".to_string(),
			Bone::new(
				Affine3A::from_translation(Vec3::Y * props.thigh_length),
				Some("right_thigh".to_string()),
			),
		),
		(
			"right_foot".to_string(),
			Bone::new(
				Affine3A::from_rotation_translation(Quat::from_rotation_x(-PI / 2.0) * Quat::from_rotation_y(PI / 8.0), Vec3::Y * props.calf_length),
				Some("right_calf".to_string()),
			),
		),
		(
			"midriff".to_string(),
			Bone::new(
				Affine3A::IDENTITY,
				Some("hips".to_string()),
			),
		),
		(
			"chest".to_string(),
			Bone::new(
				Affine3A::from_translation(Vec3::Y * props.torso_height * 0.66),
				Some("midriff".to_string()),
			),
		),
		(
			"neck".to_string(),
			Bone::new(
				Affine3A::from_translation(Vec3::Y * props.torso_height * 0.34),
				Some("chest".to_string()),
			)
		),
		(
			"head".to_string(),
			Bone::new(
				Affine3A::from_translation(Vec3::Y * props.neck_length),
				Some("neck".to_string()),
			),
		),
		(
			"left_upper_arm".to_string(),
			Bone::new(
				Affine3A::from_rotation_translation(Quat::from_rotation_z(-PI / 2.0), Vec3::new(props.shoulder_width, props.torso_height * 0.34, 0.0)),
				Some("chest".to_string()),
			),
		),
		(
			"left_lower_arm".to_string(),
			Bone::new(
				Affine3A::from_translation(Vec3::Y * props.upper_arm_length),
				Some("left_upper_arm".to_string()),
			),
		),
		(
			"left_hand".to_string(),
			Bone::new(
				Affine3A::from_translation(Vec3::Y * props.lower_arm_length),
				Some("left_lower_arm".to_string()),
			),
		),
		(
			"right_upper_arm".to_string(),
			Bone::new(
				Affine3A::from_rotation_translation(Quat::from_rotation_z(PI / 2.0), Vec3::new(-props.shoulder_width, props.torso_height * 0.34, 0.0)),
				Some("chest".to_string()),
			),
		),
		(
			"right_lower_arm".to_string(),
			Bone::new(
				Affine3A::from_translation(Vec3::Y * props.upper_arm_length),
				Some("right_upper_arm".to_string()),
			),
		),
		(
			"right_hand".to_string(),
			Bone::new(
				Affine3A::from_translation(Vec3::Y * props.lower_arm_length),
				Some("right_lower_arm".to_string()),
			),
		),
	]))
}

pub fn build_humanoid_pose(props: HumanoidProportions, pose: HumanoidPoseDescriptor, skel: &Skeleton) -> Pose {
	let torso_pose = build_torso_only_pose(&props, &pose);
	let torso_final_transforms = skel.pose(&torso_pose);

	let (left_hip_rotation, left_knee_rotation, left_foot_rotation) = leg_ik(
		props,
		torso_final_transforms["left_thigh"].transform_point3(Vec3::ZERO),
		pose.left_foot_position,
		pose.left_toes_vertical_offset
	);
	let (right_hip_rotation, right_knee_rotation, right_foot_rotation) = leg_ik(
		props,
		torso_final_transforms["right_thigh"].transform_point3(Vec3::ZERO),
		pose.right_foot_position,
		pose.right_toes_vertical_offset
	);
	let (left_shoulder_rotation, left_elbow_rotation) = arm_ik(
		props,
		torso_final_transforms["left_upper_arm"].transform_vector3(Vec3::Y),
		torso_final_transforms["left_upper_arm"].transform_point3(Vec3::ZERO),
		pose.left_hand_position,
	);
	let (right_shoulder_rotation, right_elbow_rotation) = arm_ik(
		props,
		torso_final_transforms["right_upper_arm"].transform_vector3(Vec3::Y),
		torso_final_transforms["right_upper_arm"].transform_point3(Vec3::ZERO),
		pose.right_hand_position,
	);

	Pose::new(HashMap::from([
		(
			"hips".to_string(),
			torso_pose.bones["hips"],
		),
		(
			"left_thigh".to_string(),
			Affine3A::from_quat(left_hip_rotation),
		),
		(
			"left_calf".to_string(),
			Affine3A::from_quat(left_knee_rotation),
		),
		(
			"left_foot".to_string(),
			Affine3A::from_quat(left_foot_rotation),
		),
		(
			"right_thigh".to_string(),
			Affine3A::from_quat(right_hip_rotation),
		),
		(
			"right_calf".to_string(),
			Affine3A::from_quat(right_knee_rotation),
		),
		(
			"right_foot".to_string(),
			Affine3A::from_quat(right_foot_rotation),
		),
		(
			"midriff".to_string(),
			torso_pose.bones["midriff"],
		),
		(
			"chest".to_string(),
			torso_pose.bones["chest"],
		),
		(
			"neck".to_string(),
			torso_pose.bones["neck"],
		),
		(
			"head".to_string(),
			torso_pose.bones["head"],
		),
		(
			"left_upper_arm".to_string(),
			// Affine3A::IDENTITY,
			Affine3A::from_quat(left_shoulder_rotation),
		),
		(
			"left_lower_arm".to_string(),
			Affine3A::from_quat(left_elbow_rotation),
		),
		(
			"left_hand".to_string(),
			Affine3A::from_quat(pose.left_hand_rotation),
		),
		(
			"right_upper_arm".to_string(),
			Affine3A::from_quat(right_shoulder_rotation),
		),
		(
			"right_lower_arm".to_string(),
			Affine3A::from_quat(right_elbow_rotation),
		),
		(
			"right_hand".to_string(),
			Affine3A::from_quat(pose.right_hand_rotation),
		),
	]))
}

fn build_torso_only_pose(props: &HumanoidProportions, pose: &HumanoidPoseDescriptor) -> Pose {
	let base_hip_height = props.thigh_length + props.calf_length;

	Pose::new(HashMap::from([
		(
			"hips".to_string(),
			Affine3A::from_rotation_translation(pose.hip_rotation, Vec3::Y * (pose.hip_height - base_hip_height)),
		),
		(
			"left_thigh".to_string(),
			Affine3A::IDENTITY,
		),
		(
			"left_calf".to_string(),
			Affine3A::IDENTITY,
		),
		(
			"left_foot".to_string(),
			Affine3A::IDENTITY,
		),
		(
			"right_thigh".to_string(),
			Affine3A::IDENTITY,
		),
		(
			"right_calf".to_string(),
			Affine3A::IDENTITY,
		),
		(
			"right_foot".to_string(),
			Affine3A::IDENTITY,
		),
		(
			"midriff".to_string(),
			Affine3A::from_quat(pose.chest_rotation / 2.0),
		),
		(
			"chest".to_string(),
			Affine3A::from_quat(pose.chest_rotation / 2.0),
		),
		(
			"neck".to_string(),
			Affine3A::from_quat(pose.head_rotation / 2.0),
		),
		(
			"head".to_string(),
			Affine3A::from_quat(pose.head_rotation / 2.0),
		),
		(
			"left_upper_arm".to_string(),
			Affine3A::IDENTITY,
		),
		(
			"left_lower_arm".to_string(),
			Affine3A::IDENTITY,
		),
		(
			"left_hand".to_string(),
			Affine3A::IDENTITY,
		),
		(
			"right_upper_arm".to_string(),
			Affine3A::IDENTITY,
		),
		(
			"right_lower_arm".to_string(),
			Affine3A::IDENTITY,
		),
		(
			"right_hand".to_string(),
			Affine3A::IDENTITY,
		),
	]))
}

fn leg_ik(
	props: HumanoidProportions,
	hip_pos: Vec3,
	foot_pos: Vec3,
	toe_vertical_offset: f32,
) -> (Quat, Quat, Quat) {
	assert!(props.foot_length.abs() > toe_vertical_offset.abs(),
		"The toe_vertical_offset of {} is longer than the foot length of {}", toe_vertical_offset, props.foot_length
	);

	let leg_length = props.thigh_length + props.calf_length;
	let thigh_length_sqr = props.thigh_length * props.thigh_length;
	let calf_length_sqr = props.calf_length * props.calf_length;

	let foot_offset = foot_pos - hip_pos;
	let mut foot_range_sqr = foot_offset.length_squared();
	let mut foot_range = foot_range_sqr.sqrt();
	let foot_dir = foot_offset / foot_range;
	if foot_range > leg_length {
		foot_range = leg_length;
		foot_range_sqr = foot_range * foot_range;
	}
	let hip_rotation =
		Quat::from_rotation_arc(Vec3::NEG_Y, foot_dir) *
		Quat::from_rotation_x(-(
			(foot_range_sqr + thigh_length_sqr - calf_length_sqr) /
			(2.0 * foot_range * props.thigh_length)
		).acos());
	let knee_rotation = Quat::from_rotation_x(PI - (
		(thigh_length_sqr + calf_length_sqr - foot_range_sqr) /
		(2.0 * props.thigh_length * props.calf_length)
	).acos());
	let foot_rotation =
		hip_rotation.inverse() *
		knee_rotation.inverse() *
		Quat::from_rotation_arc(Vec3::Z,
			Vec3::new(
				0.0,
				toe_vertical_offset,
				(props.foot_length * props.foot_length - toe_vertical_offset * toe_vertical_offset).sqrt(),
			).normalize()
		);

	(hip_rotation, knee_rotation, foot_rotation)
}

fn arm_ik(
	props: HumanoidProportions,
	shoulder_dir: Vec3,
	shoulder_pos: Vec3,
	hand_pos: Vec3,
) -> (Quat, Quat) {
	let arm_length = props.lower_arm_length + props.upper_arm_length;
	let upper_arm_length_sqr = props.upper_arm_length * props.upper_arm_length;
	let lower_arm_length_sqr = props.lower_arm_length * props.lower_arm_length;

	let hand_offset = hand_pos - shoulder_pos;
	let mut hand_range_sqr = hand_offset.length_squared();
	let mut hand_range = hand_range_sqr.sqrt();
	let hand_dir = hand_offset / hand_range;
	if hand_range > arm_length {
		hand_range = arm_length;
		hand_range_sqr = arm_length * arm_length;
	}
	let shoulder_rotation =
		Quat::from_rotation_arc(shoulder_dir, hand_dir) *
		Quat::from_rotation_x(-(
			(hand_range_sqr + upper_arm_length_sqr - lower_arm_length_sqr) /
			(2.0 * hand_range * props.upper_arm_length)
		).acos());
	let elbow_rotation = Quat::from_rotation_x(PI - (
		(upper_arm_length_sqr + lower_arm_length_sqr - hand_range_sqr) /
		(2.0 * props.upper_arm_length * props.lower_arm_length)
	).acos());

	(shoulder_rotation, elbow_rotation)
}
