use de_mosaic_rs::helpers::img::load_image;

fn main() {
    match load_image("d1.jpeg") {
        Ok(img) => println!("Image loaded successfully!"),
        Err(e) => println!("Failed to load image: {:?}", e),
    }
}
