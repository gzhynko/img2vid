use std::fs;
use std::io::Read;
use std::path::Path;
use image::RgbImage;
use img2vid::{images_to_video};

const IMAGES_DIR: &str = "assets/images";
const IMAGE_FILE_TYPE: &str = "jpg";

fn main() {
    let mut images = Vec::<Vec<u8>>::new();
    for entry_res in fs::read_dir(Path::new(IMAGES_DIR)).unwrap() {
        if let Ok(entry) = entry_res {
            if entry.file_type().unwrap().is_file() {
                if entry.path().extension().unwrap().to_str().unwrap() == IMAGE_FILE_TYPE {
                    let image = image::io::Reader::open(entry.path()).unwrap().decode().unwrap().to_rgb8();
                    let data = image.bytes().map(|byte| {byte.unwrap()}).collect::<Vec<u8>>();
                    images.push(data);
                }
            }
        }
    }

    let buf = images_to_video("assets/images/*.jpg", "mpeg4","mjpeg", 24);
    std::fs::write(Path::new("assets/output.mp4"), buf).unwrap();
}