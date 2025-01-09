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

const GREEN_RED: [[f32; 5]; 5] = [
    [0.0, 0.0, -1.0, 0.0, 0.0],
    [0.0, 0.0, 2.0, 0.0, 0.0],
    [-1.0, 2.0, 4.0, 2.0, -1.0],
    [0.0, 0.0, 2.0, 0.0, 0.0],
    [0.0, 0.0, -1.0, 0.0, 0.0]
];

const GREEN_BLUE: [[f32; 5]; 5] = GREEN_RED;

const RED_GREEN: [[f32; 5]; 5] = [
    [0.0, 0.0, 0.5, 0.0, 0.0],
    [0.0, -1.0, 0.0, -1.0, 0.0],
    [-1.0, 4.0, 5.0, 4.0, -1.0],
    [0.0, -1.0, 0.0, -1.0, 0.0],
    [0.0, 0.0, 0.5, 0.0, 0.0]
];

const BLUE_GREEN: [[f32; 5]; 5] = RED_GREEN;

const RED_BLUE: [[f32; 5]; 5] = [
    [0.0, 0.0, -1.5, 0.0, 0.0],
    [0.0, 2.0, 0.0, 2.0, 0.0],
    [-1.5, 0.0, 6.0, 0.0, -1.5],
    [0.0, 2.0, 0.0, 2.0, 0.0],
    [0.0, 0.0, -1.5, 0.0, 0.0]
];

const BLUE_RED: [[f32; 5]; 5] = RED_BLUE;

pub fn process_image(input_path: &Path, output_path: &Path) {
    let img = image::open(input_path).expect("Failed to open image");
    let (width, height) = img.dimensions();
    let input_img = img.into_rgb8();
    
    let mut output_img: RgbImage = ImageBuffer::new(width, height);
    
    // Process each pixel
    for y in 0..height {
        for x in 0..width {
            let pixel = input_img.get_pixel(x, y);
            
            // Get values from surrounding pixels
            let mut neighbors = Vec::new();
            
            // Add surrounding pixels if they exist
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    
                    if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                        neighbors.push(*input_img.get_pixel(nx as u32, ny as u32));
                    }
                }
            }
            
            // Calculate average RGB values from neighbors
            let mut r_sum = 0u32;
            let mut g_sum = 0u32;
            let mut b_sum = 0u32;
            
            for p in &neighbors {
                r_sum += p[0] as u32;
                g_sum += p[1] as u32;
                b_sum += p[2] as u32;
            }
            
            let count = neighbors.len() as u32;
            let r = (r_sum / count) as u8;
            let g = (g_sum / count) as u8;
            let b = (b_sum / count) as u8;
            
            output_img.put_pixel(x, y, image::Rgb([r, g, b]));
        }
    }
    
    output_img.save(output_path).expect("Failed to save image");
}

// Helper function to debug pixel values
fn print_pixel_info(x: u32, y: u32, pixel: &[u8]) {
    println!("Pixel at ({}, {}): R={}, G={}, B={}", x, y, pixel[0], pixel[1], pixel[2]);
}

fn apply_kernel(buffer: &[u8], x: u32, y: u32, width: u32, kernel: &[[f32; 5]; 5], channel: usize, normalizer: f32) -> f32 {
    let mut sum = 0.0f32;
    let center_x = x as i32;
    let center_y = y as i32;
    
    for ky in 0..5 {
        for kx in 0..5 {
            let sample_x = center_x + (kx as i32) - 2;
            let sample_y = center_y + (ky as i32) - 2;
            
            // Bounds checking is not needed because of padded buffer with mirrored edges
            let idx = ((sample_y as u32 * width + sample_x as u32) * 3 + channel as u32) as usize;
            sum += buffer[idx] as f32 * kernel[ky][kx];
        }
    }
    
    sum / normalizer
}

fn get_original_value(buffer: &[u8], x: u32, y: u32, width: u32, channel: usize) -> f32 {
    let idx = ((y * width + x) * 3 + channel as u32) as usize;
    buffer[idx] as f32
}

fn clamp(value: f32) -> f32 {
    value.max(0.0).min(255.0)
}


// pub fn process_image(input_path: &Path, output_path: &Path) {
//     let img = image::open(input_path).expect("Failed to open image");
//     let (width, height) = img.dimensions();
    
//     let rgb_img = img.into_rgb8();
//     let mut output_img: RgbImage = ImageBuffer::new(width, height);
    
//     for (x, y, pixel) in rgb_img.enumerate_pixels() {
//         output_img.put_pixel(x, y, *pixel);
//     }
//     output_img.save(output_path).expect("Failed to save image");
// }