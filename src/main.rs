mod args;
use args::Args;
use image::{io::Reader, DynamicImage, ImageFormat};




fn main() {
    let args = Args::new();
    println!("{:?}", args);
}

//creating a function named find_image_from_path to open the image file from a path argument

fn find_image_from_path(path: String) ->(DynamicImage, ImageFormat){
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}
// The image and image_format variables are returned as a tuple