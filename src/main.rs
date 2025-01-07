use de_mosaic_rs::helpers::img::{load_image};
use de_mosaic_rs::{ Mosaic };

fn main() {
    match load_image("d1.jpeg") {
        Ok(img) => println!("Image loaded successfully!"),
        Err(e) => println!("Failed to load image: {:?}", e),
    }

    let m = Mosaic::new("input", "output");
    let imgs = m.select();
    m.process(&imgs);
    println!("{:?}", imgs);
}
