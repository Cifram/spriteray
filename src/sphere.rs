use glam::Vec3;

use crate::{SdfResult, Color};

pub fn sphere(pos: Vec3, radius: f32, color: Color) -> SdfResult {
  SdfResult {
    range: pos.length() - radius,
    normal: pos.normalize_or_zero(),
    color,
  }
}
