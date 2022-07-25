use glam::{Vec3, Affine3A, Quat};

use crate::{Color, SdfFn, transform, infinite_cylinder, difference3, plane};

pub fn cylinder(radius: f32, height: f32, color: Color) -> SdfFn {
	difference3(
		infinite_cylinder(radius, color),
		transform(
			Affine3A::from_scale(Vec3::new(1.0, 1.0, 1.0)),
			plane(color),
		),
		transform(
			Affine3A::from_rotation_translation(Quat::from_rotation_x(std::f32::consts::PI), Vec3::Y * height),
			plane(color),
		),
	)
}
