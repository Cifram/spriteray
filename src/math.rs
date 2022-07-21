use glam::Vec2;

#[derive(Copy, Clone)]
pub struct Line {
	pub normal: Vec2,
	pub offset: f32,
}

impl Line {
	pub fn from_point_normal(point: Vec2, normal: Vec2) -> Self {
		Self {
			normal,
			offset: -point.dot(normal),
		}
	}

	pub fn range(self, point: Vec2) -> f32 {
		point.dot(self.normal) + self.offset
	}
}
