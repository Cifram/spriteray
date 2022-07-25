use glam::{Vec3, Affine3A, Quat};

use crate::{Color, SdfFn, transform, difference6, plane};

const PI: f32 = std::f32::consts::PI;

pub fn cube(width: f32, color: Color) -> SdfFn {
	difference6(
		transform(
			Affine3A::from_scale(Vec3::new(1.0, -1.0, 1.0)),
			plane(color),
		),
		transform(
			Affine3A::from_rotation_translation(Quat::from_rotation_x(PI), Vec3::Y * width),
			plane(color),
		),
		transform(
			Affine3A::from_rotation_translation(Quat::from_rotation_x(PI / 2.0), Vec3::Y * width / 2.0),
			plane(color),
		),
		transform(
			Affine3A::from_rotation_translation(Quat::from_rotation_x(-PI / 2.0), Vec3::Y * width / 2.0),
			plane(color),
		),
		transform(
			Affine3A::from_rotation_translation(Quat::from_rotation_z(PI / 2.0), Vec3::Y * width / 2.0),
			plane(color),
		),
		transform(
			Affine3A::from_rotation_translation(Quat::from_rotation_z(-PI / 2.0), Vec3::Y * width / 2.0),
			plane(color),
		)
	)
}
