use opencv::prelude::*;
use opencv::{videoio, highgui, imgcodecs, core};
use std::fs;
use crate::imgcodecs::ImwriteFlags::IMWRITE_JPEG_QUALITY;

fn main() -> opencv::Result<()> {
	let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L2)?;
	if !videoio::VideoCapture::is_opened(&cam)? {
		panic!("The cam did not open");
	}
	let mut i = 0;
	fs::create_dir("/screenshots");
	loop {
		let mut frame = Mat::default();
		cam.read(&mut frame)?;
		highgui::imshow("cam", &frame)?;

		if highgui::wait_key(10)? == 's' as i32 {
			let params = opencv::core::Vector::default();
			imgcodecs::imwrite(&format!("img{i}.jpeg"), &frame, &params);
			i += 1
		}
		if highgui::wait_key(10)? == 'q' as i32 {
			break;
		}
	}
	highgui::destroy_all_windows()?;
	Ok(())
}
