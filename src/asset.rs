mod image;
mod video;

pub fn image_png() -> String {
    format!("Static Image {}", image::png_files())
}

pub fn video_mp4() -> String {
    format!("Static Video {}", video::mp4_files())
}