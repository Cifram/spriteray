use glam::{Vec3, Affine3A, Quat};

use crate::{Color, transform, SdfFn, difference2, cone, plane_normal_offset};

pub fn truncated_cone(base_radius: f32, top_radius: f32, height: f32, color: Color) -> SdfFn {
	let (base_radius, top_radius, cone_transform) = if base_radius > top_radius {
		(base_radius, top_radius, Affine3A::IDENTITY)
	} else {
		(top_radius, base_radius, Affine3A::from_scale_rotation_translation(Vec3::new(1.0, -1.0, 1.0), Quat::IDENTITY, Vec3::Y * height))
	};
	let full_height = (height * base_radius) / (base_radius - top_radius);
	transform(
		cone_transform,
		difference2(
			cone(base_radius, full_height, color),
			plane_normal_offset(Vec3::NEG_Y, height, color),
		),
	)
}
