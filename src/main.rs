use opencv::prelude::*;
use opencv::{videoio, highgui, imgcodecs};
use std::fs;
use std::path::Path;

const SCREENSHOT_CODE: i32 = 115; // 's'
const QUIT_CODE: i32 = 113; // 'q'

fn main() -> opencv::Result<()> {
	let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L2)?;
	if !videoio::VideoCapture::is_opened(&cam)? {
		panic!("The cam did not open");
	}
	if !Path::new("./screenshots").exists() {
		fs::create_dir("./screenshots").expect("Directory cannot be created");
	}
	
	let mut screenshot_index = 0;
	while cam.is_opened()? {
		let mut frame = Mat::default();
		cam.read(&mut frame)?;
		highgui::imshow("cam", &frame)?;


		let key = highgui::wait_key(5)?;
		match key {
			SCREENSHOT_CODE => {
				let params = opencv::core::Vector::default();
				imgcodecs::imwrite(&format!("./screenshots/img{screenshot_index}.jpeg"),
							   &frame, &params)?;
				screenshot_index += 1
			},
			QUIT_CODE => {
				cam.release()?;
			},
			_ => {

			},
		}
	}

	highgui::destroy_all_windows()?;
	Ok(())
}
