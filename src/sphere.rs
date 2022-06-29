use glam::Vec3;

use crate::{SdfResult, Color};

pub fn sphere(pos: Vec3, center: Vec3, radius: f32, color: Color) -> SdfResult {
  let range = (pos - center).length() - radius;
  if range > 0.0 {
    SdfResult::Miss { range }
  } else {
    SdfResult::Hit { range, normal: (pos - center).normalize_or_zero(), color }    
  }
}
