use std::fs;
use std::io::Read;
use std::path::Path;
use image::RgbImage;
use img2vid::{images_to_video};

const IMAGES_DIR: &str = "assets/images";

fn main() {
    let video_buffer = images_to_video(&format!("{}/*.jpg", IMAGES_DIR), "mpeg4","mjpeg", 24);
    fs::write(Path::new("assets/output.mp4"), video_buffer).unwrap();
}