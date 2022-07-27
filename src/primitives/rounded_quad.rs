use glam::{Affine3A, Quat, Vec3};

use crate::{union5, transform, capsule, Color, SdfFn, plane_normal_point, plane_normal_offset, intersection6};

const PI: f32 = std::f32::consts::PI;

pub fn rounded_quad(bottom_width: f32, top_width: f32, height: f32, bottom_depth: f32, top_depth: f32, color: Color) -> SdfFn {
	let width_change = bottom_width / 2.0 - top_width / 2.0;
	let side_angle = (width_change / height).atan();
	let side_length = (width_change * width_change + height * height).sqrt();
	let plane_angle = ((bottom_depth - top_depth) / height).acos() - PI/2.0;
	let front_plane_point = Quat::from_rotation_x(plane_angle) * Vec3::Z * bottom_depth;
	let front_plane_normal = front_plane_point.normalize();
	let back_plane_point = front_plane_point * Vec3::new(1.0, 1.0, -1.0);
	let back_plane_normal = front_plane_normal * Vec3::new(1.0, 1.0, -1.0);
	union5(
		transform(
			Affine3A::from_rotation_translation(Quat::from_rotation_z(PI/2.0), Vec3::Y * bottom_width/2.0),
			capsule(bottom_depth, bottom_depth, bottom_width, color),
		),
		transform(
			Affine3A::from_rotation_translation(Quat::from_rotation_z(PI/2.0), Vec3::new(height, top_width/2.0, 0.0)),
			capsule(top_depth, top_depth, top_width, color),
		),
		transform(
			Affine3A::from_translation(Vec3::X * bottom_width/2.0),
			transform(
				Affine3A::from_rotation_z(side_angle),
				capsule(bottom_depth, top_depth, side_length, color),
			)
		),
		transform(
			Affine3A::from_translation(Vec3::NEG_X * bottom_width/2.0),
			transform(
				Affine3A::from_rotation_z(-side_angle),
				capsule(bottom_depth, top_depth, side_length, color),
			)
		),
		intersection6(
			plane_normal_point(front_plane_normal, front_plane_point, color),
			plane_normal_point(back_plane_normal, back_plane_point, color),
			plane_normal_point(Quat::from_rotation_z(-side_angle) * Vec3::NEG_X, Vec3::NEG_X * bottom_width / 2.0, color),
			plane_normal_point(Quat::from_rotation_z(side_angle) * Vec3::X, Vec3::X * bottom_width / 2.0, color),
			plane_normal_offset(Vec3::NEG_Y, 0.0, color),
			plane_normal_offset(Vec3::Y, -height, color),
		)
	)
}
