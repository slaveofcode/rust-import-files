use crate::asset::{image_png};

pub fn two_content() -> String {
    "Two Content".to_string()
}

pub fn content_with_image() -> String {
    format!("Content with Image: {}", image_png()).to_string()
}