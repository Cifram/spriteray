use glam::Vec3;

use crate::{SdfResult, Color, cone, difference2, plane};

pub fn truncated_cone(pos: Vec3, base_radius: f32, top_radius: f32, height: f32, color: Color) -> SdfResult {
  let full_height = (height * base_radius) / (base_radius - top_radius);
	difference2(pos,
		&|pos| cone(pos, base_radius, full_height, color),
		&|pos| plane(Vec3::new(pos.x, -pos.y + height, pos.z), color)
	)
}
