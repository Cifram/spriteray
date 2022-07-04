#[derive(Clone, Copy)]
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

	pub fn darken(self, amount: f32) -> Self {
		Color::new(self.r * (1.0 - amount), self.g * (1.0 - amount), self.b * (1.0 - amount))
	}

	pub fn brighten(self, amount: f32) -> Self {
		Color::new(
			self.r + (1.0 - self.r) * amount,
			self.g + (1.0 - self.g) * amount,
			self.b + (1.0 - self.b) * amount,
		)
	}
}
