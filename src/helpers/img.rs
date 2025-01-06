use std::path::PathBuf;
use image::{ImageError, DynamicImage};

pub fn load_image(image_name: &str) -> Result<DynamicImage, ImageError> {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let image_path = PathBuf::from(project_root)
        .join("assets")
        .join(image_name);
    
    println!("Trying to load image from: {:?}", image_path);
    image::open(image_path)
}