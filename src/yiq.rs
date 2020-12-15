use image::{ Pixel, Rgba };

fn get_rgb(pix: &Rgba<u8>) -> (f64,f64,f64) {
	let rgba = pix.channels();
	(rgba[0] as f64, rgba[1] as f64, rgba[2] as f64)
}

fn rgb2y(r: f64, g: f64, b: f64) -> f64 {
	r*0.29889531 + g*0.58662247 + b*0.11448223
}

fn rgb2i(r: f64, g: f64, b: f64) -> f64 {
	r*0.59597799 - g*0.27417610 - b*0.32180189
}

fn rgb2q(r: f64, g: f64, b: f64) -> f64 {
	r*0.21147017 - g*0.52261711 + b*0.31114694
}

pub fn delta(pix1: &Rgba<u8>, pix2: &Rgba<u8>) -> f64 {
	let (r1, g1, b1) = get_rgb(pix1);
	let (r2, g2, b2) = get_rgb(pix2);

	let y = rgb2y(r1, g1, b1) - rgb2y(r2, g2, b2);
	let i = rgb2i(r1, g1, b1) - rgb2i(r2, g2, b2);
	let q = rgb2q(r1, g1, b1) - rgb2q(r2, g2, b2);

	return 0.5053*y*y + 0.299*i*i + 0.1957*q*q
}

pub const MAX_DELTA: f64 = 35215.0;