use opencv::prelude::*;
use opencv::{videoio, highgui};
fn main() -> opencv::Result<()> {
	let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L2)?;
	if !videoio::VideoCapture::is_opened(&cam)? {
		panic!("The cam did not open");
	}
	loop {
		let mut frame = Mat::default();
		cam.read(&mut frame)?;
		highgui::imshow("cam", &frame)?;
		if highgui::wait_key(10)? == 'q' as i32 {
			break;
		}
	}
	highgui::destroy_all_windows()?;
	Ok(())
}
