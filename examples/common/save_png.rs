pub fn save_png(name: &str, bytes: &[u8], width: u32, height: u32) {
	let path = format!("example_images/{}.png", name);
	std::fs::create_dir_all("example_images/").unwrap();
	image::save_buffer(path, bytes, width, height, image::ColorType::Rgba8).unwrap();
}
