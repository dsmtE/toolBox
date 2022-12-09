use clap::Parser;
use image::{GenericImageView, imageops};
use std::path::PathBuf;
use std::io::Write;
use glob::glob;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(help("image input path"))]
    source_glob_pattern: String,
    dst_image_path: PathBuf,

    #[arg(short, long, default_value_t = 1200)]
    max_width: u32
}

fn mozjpeg_save(width: u32, height: u32, image_slice: &[u8], path: & impl AsRef<std::path::Path>) -> Result<(), std::io::Error>{

    let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);
    
    comp.set_size(width as _, height as _);
    comp.set_mem_dest();
    comp.start_compress();
    
    assert!(comp.write_scanlines(image_slice));
    
    comp.finish_compress();
    let jpeg_bytes_slice = comp.data_as_mut_slice().expect("Unable to xrite buffer to moz compressor");

    let file = std::fs::File::create(path)?;

    std::io::BufWriter::new(file).write_all(jpeg_bytes_slice)
}

fn resize(img: image::DynamicImage, max_width: u32, filter: Option<imageops::FilterType>) -> image::DynamicImage {

    let (width, height) = img.dimensions();
    let target_width = std::cmp::min(max_width, width);
    let target_height = (target_width as f32/width as f32 * height as f32) as u32;
    
    img.resize(target_width, target_height, filter.unwrap_or(imageops::Lanczos3))
}

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);

    for entry in glob(&args.source_glob_pattern).expect("Failed to read glob pattern") {
        match entry {
            Err(e) => println!("{:?}", e),
            Ok(path) => {
                println!("path found : {:?}", path.display());
                let target_path = args.dst_image_path.join(path.file_name().unwrap());

                let mut img = image::open(&path)
                    .expect(format!("Unale to open the given image at path {}", path.display()).as_str());
                
                print!("Resizing");
                img = resize(img, args.max_width, None);
                println!(", output size : {}x{}", img.width(), img.height());
                
                println!("Saving at {}", target_path.clone().display());
                // img.save(args.dst_image_path.as_path()).expect("Unable to save image at destination path");
                mozjpeg_save(img.width(), img.height(), img.as_flat_samples_u8().unwrap().as_slice(), &target_path)
                    .expect("Unable to save the file");
            },
        }
    }

}