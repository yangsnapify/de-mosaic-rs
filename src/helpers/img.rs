use std::path::{PathBuf, Path};
use image::{ImageError, DynamicImage, RgbImage, ImageBuffer};
use image::GenericImageView;

pub fn load_image(image_name: &str) -> Result<DynamicImage, ImageError> {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let image_path = PathBuf::from(project_root)
        .join("assets")
        .join(image_name);
    
    println!("Trying to load image from: {:?}", image_path);
    image::open(image_path)
}

pub fn process_image(input_path: &Path, output_path: &Path) {
    let img = image::open(input_path).expect("Failed to open image");
    let (width, height) = img.dimensions();
    
    let rgb_img = img.into_rgb8();
    let mut output_img: RgbImage = ImageBuffer::new(width, height);
    
    for (x, y, pixel) in rgb_img.enumerate_pixels() {
        output_img.put_pixel(x, y, *pixel);
    }
    output_img.save(output_path).expect("Failed to save image");
}