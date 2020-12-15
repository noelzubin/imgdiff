use imgdiff::generate_diff;
use structopt::StructOpt;
use std::path::PathBuf;
use image::io::Reader;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name="image1")]
    inp1: PathBuf,

    #[structopt(name="image2")]
    inp2: PathBuf,

    #[structopt(name="output")]
    out: PathBuf,

    #[structopt(short, long="threshold")]
    threshold: Option<f64>,
}

fn main() {
    let opt = Opt::from_args();
    let img1 = Reader::open(opt.inp1).expect("Error reading file a").decode().expect("Error decoding img1");
    let img2 = Reader::open(opt.inp2).expect("Error reading file a").decode().expect("Error decoding img1");

    let threshold = opt.threshold.unwrap_or(0.01);
    if threshold < 0.0 || threshold > 1.0 { panic!("Threshold should be between 0 and 1"); }

    generate_diff(img1, img2, threshold, opt.out);
}
