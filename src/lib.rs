use std::io::Read;
use std::process::Command;

pub fn images_to_video(images_glob_pattern: &str, video_codec: &str, video_format: &str, video_fps: u32) -> Vec<u8> {
    let mut command = Command::new("ffmpeg");
    command
        .args(["-f", "image2"])
        .args(["-pattern_type", "glob"])
        .args(["-framerate", &video_fps.to_string()])
        .args(["-i", images_glob_pattern])
        .args(["-vcodec", video_codec])
        .args(["-f", video_format])
        .args(["-"]);

    let (mut stdout_reader, stdout_writer) = os_pipe::pipe().unwrap();
    command.stdout(stdout_writer);

    let mut handle = command.spawn().unwrap();
    drop(command);

    let mut video_buffer = Vec::new();
    stdout_reader.read_to_end(&mut video_buffer).unwrap();

    handle.wait().unwrap();

    video_buffer
}
