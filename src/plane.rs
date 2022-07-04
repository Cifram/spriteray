use glam::Vec3;

use crate::{SdfResult, Color};

pub fn plane(pos: Vec3, color: Color) -> SdfResult {
  SdfResult {
    range: pos.y,
    normal: Vec3::Y,
    color,
  }
}
