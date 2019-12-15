use std::fs::File;
use std::io::BufReader;
use rodio::{self, Source};

struct Sound{
	device: rodio::Device
}
impl Sound{
	pub fn new() -> Self {
		let device = rodio::default_output_device().unwrap();
		Self {
			device
		}
	}
}
