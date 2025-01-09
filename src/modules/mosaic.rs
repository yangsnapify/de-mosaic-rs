use image::{ GrayImage, Luma, RgbImage, open };
use walkdir::WalkDir;
use std::path::{ Path, PathBuf };
use indicatif::{ ProgressBar, ProgressStyle };
use crate::helpers::{ process_image };

pub struct Mosaic {
    input_path: String,
    output_path: String,
    temp_path: String,
}

impl Mosaic {
    pub fn new(input: impl Into<String>, output: impl Into<String>) -> Mosaic {
        let _input: String = input.into();
        let _output: String = output.into();
        let _temp: String = "/temp".to_string();

        Mosaic {
            input_path: _input,
            output_path: _output,
            temp_path: _temp,
        }
    }

    pub fn select(&self) -> Vec<PathBuf> {
        let image_paths: Vec<PathBuf> = WalkDir::new(self.input_path.clone())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e|
                e
                    .path()
                    .extension()
                    .map_or(false, |ext| { ext == "png" || ext == "jpg" || ext == "jpeg" })
            )
            .map(|e| e.path().to_path_buf())
            .collect();
        image_paths
    }

    pub fn process(&self, paths: &Vec<PathBuf>) {
        let progress_bar = ProgressBar::new(paths.len() as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:30.green/grey}] {pos}/{len} {percent}%")
                .unwrap(),
        );
    
        for input_path in paths {
            let output_path = Path::new(&self.output_path).join(input_path.file_name().unwrap());
            process_image(input_path, &output_path);
            progress_bar.inc(1);
        }
    
        progress_bar.finish_with_message("Processing complete");
    }

    pub fn init_progress() {}
}
