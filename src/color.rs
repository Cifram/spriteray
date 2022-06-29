pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
}

impl Color {
	pub fn new(r: f32, g: f32, b: f32) -> Self {
		Color { r, g, b }
	}

	pub fn bytes(self) -> [u8; 3] {
		[
			(self.r * 255.0) as u8,
			(self.g * 255.0) as u8,
			(self.b * 255.0) as u8,
		]
	}
}
