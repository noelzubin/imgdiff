use image::{ DynamicImage, GenericImageView, ColorType };
use std::path::PathBuf;
mod yiq;


fn has_same_dimensions(img1: &DynamicImage, img2: &DynamicImage) -> bool { 
  img1.dimensions() == img2.dimensions()
}

pub fn generate_diff(img1: DynamicImage, img2: DynamicImage, threshold: f64, out: PathBuf) {
  if !has_same_dimensions(&img1, &img2) {
    panic!("Images have different sizes");
  }

  let img1 = img1.to_rgba8();
  let img2 = img2.to_rgba8();

  let (width,height) = img1.dimensions();
  let mut buf: Vec<u8> = Vec::with_capacity((width*height*4) as usize);

  let max_delta = yiq::MAX_DELTA * threshold;

  img1.pixels().zip(img2.pixels()).for_each(|(pix1, pix2)| {
    buf.push(255);
    buf.push(0);
    buf.push(0);

    if yiq::delta(pix1, pix2) >= max_delta {
        buf.push(255);
    } else {
        buf.push(0);
    }
  });

  image::save_buffer(out, &buf, width, height, ColorType::Rgba8).unwrap()
}