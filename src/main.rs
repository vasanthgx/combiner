mod args;
use args::Args;
use image::{io::Reader, DynamicImage, ImageFormat};




fn main() -> Result<(), ImageDataErrors {
    let args = Args::new();
    println!("{:?}", args);

    let(image_1, image_1_format) = find_image_from_path(args.image_1);
    let(image_2, image_2_format) = find_image_from_path(args.image_2);

    if image_1_format != image_2_format{
        return Err(ImageDataErrors::DifferentImageFormats);
    }
    ok(());
}

//creating a function named find_image_from_path to open the image file from a path argument

fn find_image_from_path(path: String) ->(DynamicImage, ImageFormat){
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}
// The image and image_format variables are returned as a tuple


// enum definition for error messages
enum ImageDataErrors {
    DifferentImageFormats,
}