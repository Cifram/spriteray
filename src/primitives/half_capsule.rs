use glam::{Vec3, Affine3A};

use crate::{Color, SdfFn, union2, transform, truncated_cone, sphere};

pub fn half_capsule(bottom_radius: f32, top_radius: f32, height: f32, color: Color) -> SdfFn {
	let side_length = (height*height + top_radius*top_radius - bottom_radius*bottom_radius).sqrt();
	let angle = (height / top_radius).atan() + (bottom_radius / side_length).atan();
	let angle = if bottom_radius < top_radius { angle } else { std::f32::consts::PI - angle };
	let adj_height = angle.sin() * side_length;
	let adj_bottom_radius = top_radius - angle.cos() * side_length * if bottom_radius < top_radius { 1.0 } else { -1.0 };
	union2(
		sphere(bottom_radius, color),
		transform(
			Affine3A::from_translation(Vec3::NEG_Y * (height - adj_height)),
			truncated_cone(adj_bottom_radius, top_radius, adj_height, color),
		)
	)
}
