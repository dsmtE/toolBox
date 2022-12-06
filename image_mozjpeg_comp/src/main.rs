use clap::Parser;
use image::{GenericImageView, imageops};
use std::path::PathBuf;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(help("image input path"))]
    src_image_path: PathBuf,
    dst_image_path: PathBuf,

    #[arg(short, long, default_value_t = 1200)]
    max_width: u32
}

fn mozjpeg_save(width: u32, height: u32, image_slice: &[u8], path: impl AsRef<std::path::Path>) -> Result<(), std::io::Error>{

    let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);
    
    comp.set_size(width as _, height as _);
    comp.set_mem_dest();
    comp.start_compress();
    
    assert!(comp.write_scanlines(image_slice));
    
    comp.finish_compress();
    let jpeg_bytes_slice = comp.data_as_mut_slice().expect("Unable to xrite buffer to moz compressor");

    let file = std::fs::File::create(path).unwrap_or_else(|e| panic!("{}", e));

    std::io::BufWriter::new(file).write_all(jpeg_bytes_slice)
}
fn main() {
    let args = Arguments::parse();
    
    println!("{:?}", args);

    let mut img = image::open(args.src_image_path.as_path()).expect("Unale to open the given image");
    let (width, height) = img.dimensions();
    
    let target_width = std::cmp::min(args.max_width, width);
    let target_height = (target_width as f32/width as f32 * height as f32) as u32;

    println!("Resizing");
    img = img.resize(target_width, target_height, imageops::Lanczos3);
    
    println!("Saving");
    img.save(args.dst_image_path.as_path()).expect("Unable to save image at destination path");
    mozjpeg_save(target_width, target_height, img.as_flat_samples_u8().unwrap().as_slice(), args.dst_image_path)
        .expect("Unable to save the file");
    
}