use std::collections::HashMap;

use glam::{Affine3A, Vec3};

use crate::{SdfFn, SdfResult, Color, transform, union2};

#[derive(Clone)]
pub struct Bone {
	pub transform: Affine3A,
	pub parent: Option<String>,
}

impl Bone {
	pub fn new(transform: Affine3A, parent: Option<String>) -> Self {
		Self { transform, parent }
	}
}

#[derive(Clone)]
pub struct Skeleton {
	pub bones: HashMap<String, Bone>,
}

impl Skeleton {
	pub fn new(bones: HashMap<String, Bone>) -> Self {
		Self { bones }
	}

	pub fn pose(&self, pose: &Pose) -> HashMap<String, Affine3A> {
		let mut final_pose = HashMap::new();
		for (name, bone) in self.bones.iter() {
			self.set_bone_transform(name, bone, pose, &mut final_pose);
		}
		final_pose
	}

	fn set_bone_transform(&self, name: &str, bone: &Bone, pose: &Pose, final_pose: &mut HashMap<String, Affine3A>) {
		if final_pose.contains_key(name) {
			return;
		}
		let transform = bone.transform * pose.bones[name];
		if let Some(parent_name) = bone.parent.clone() {
			let parent_bone = self.bones[&parent_name].clone();
			self.set_bone_transform(&parent_name, &parent_bone, pose, final_pose);
			final_pose.insert(name.to_string(), final_pose[&parent_name] * transform);
		} else {
			final_pose.insert(name.to_string(), transform);
		}
	}
}

#[derive(Clone)]
pub struct Pose {
	pub bones: HashMap<String, Affine3A>,
}

impl Pose {
	pub fn new(bones: HashMap<String, Affine3A>) -> Self {
		for (name, transform) in bones.iter() {
			assert!(!transform.is_nan(), "Bone {} has a NaN transform", name);
		}
		Self { bones }
	}
}

pub struct Character {
	pub sdfs: Vec<(String, SdfFn)>,
}

impl Character {
	pub fn new(sdfs: Vec<(String, SdfFn)>) -> Self {
		Self { sdfs }
	}
}

pub fn skeleton(skeleton: Skeleton, pose: Pose, mut character: Character) -> SdfFn {
	let pose = skeleton.pose(&pose);
	let mut sdf: SdfFn = Box::new(|_| SdfResult { range: std::f32::MAX, normal: Vec3::Y, color: Color::new(0.0, 0.0, 0.0) });
	for (bone_name, char_sdf) in character.sdfs.drain(..) {
		sdf = union2(
			sdf,
			transform(
				pose[&bone_name],
				char_sdf,
			),
		);
	}
	sdf
}
