use glam::{Vec3, Affine3A};

use crate::{SdfResult, Color, Sphere, TruncatedCone, Union2, Transform, Sdf};

pub struct HalfCapsule {
	sdf: Union2<Sphere, Transform<TruncatedCone>>,
}

impl HalfCapsule {
	pub fn new(bottom_radius: f32, top_radius: f32, height: f32, color: Color) -> Self {
		let side_length = (height*height + top_radius*top_radius - bottom_radius*bottom_radius).sqrt();
		let angle = (height / top_radius).atan() + (bottom_radius / side_length).atan();
		let angle = if bottom_radius < top_radius { angle } else { std::f32::consts::PI - angle };
		let adj_height = angle.sin() * side_length;
		let adj_bottom_radius = top_radius - angle.cos() * side_length * if bottom_radius < top_radius { 1.0 } else { -1.0 };
		Self {
			sdf: Union2::new(
				Sphere::new(bottom_radius, color),
				Transform::new(
					Affine3A::from_translation(Vec3::NEG_Y * (height - adj_height)),
					TruncatedCone::new(adj_bottom_radius, top_radius, adj_height, color),
				),
			)
		}
	}
}

impl Sdf for HalfCapsule {
	fn check(&self, pos: Vec3) -> SdfResult {
		self.sdf.check(pos)
	}
}
