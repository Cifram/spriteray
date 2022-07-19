use glam::Vec3;

use crate::{SdfResult, Color, Sdf, SdfFn};

pub struct Torus {
	inner_radius: f32,
	outer_radius: f32,
	color: Color,
}

impl Torus {
	pub fn new(inner_radius: f32, outer_radius: f32, color: Color) -> Self {
		Self { inner_radius, outer_radius, color }
	}
}

impl Sdf for Torus {
	fn check(&self, pos: Vec3) -> SdfResult {
		if pos == Vec3::ZERO {
			return SdfResult {
				range: self.inner_radius,
				normal: Vec3::ZERO,
				color: self.color,
			}
		}
	
		let flat_dir = Vec3::new(pos.x, 0.0, pos.z).normalize();
		let center = flat_dir * (self.outer_radius + self.inner_radius) / 2.0;
		let radius = self.outer_radius - self.inner_radius;
		let offset = pos - center;
		let range = offset.length() - radius;
		SdfResult {
			range,
			normal: offset.normalize_or_zero(),
			color: self.color,
		}
	}
}

pub fn torus(inner_radius: f32, outer_radius: f32, color: Color) -> SdfFn {
	Box::new(move |pos| {
		if pos == Vec3::ZERO {
			return SdfResult {
				range: inner_radius,
				normal: Vec3::ZERO,
				color: color,
			}
		}
	
		let flat_dir = Vec3::new(pos.x, 0.0, pos.z).normalize();
		let center = flat_dir * (outer_radius + inner_radius) / 2.0;
		let radius = outer_radius - inner_radius;
		let offset = pos - center;
		let range = offset.length() - radius;
		SdfResult {
			range,
			normal: offset.normalize_or_zero(),
			color: color,
		}
	})
}
