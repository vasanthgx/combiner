mod args;
use args::Args;
use image::{io::Reader, DynamicImage, ImageFormat, imageops::Triangle};




fn main() -> Result<(), ImageDataErrors {
    let args = Args::new();
    println!("{:?}", args);

    let(image_1, image_1_format) = find_image_from_path(args.image_1);
    let(image_2, image_2_format) = find_image_from_path(args.image_2);

    if image_1_format != image_2_format{
        return Err(ImageDataErrors::DifferentImageFormats);
    }
    // ok(());

    let (image_1, image_2) = standardize_size(image_1, image_2)
    ok(())
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


// Resizing the Images to Match : we resize the larger image to match the smaller image

fn get_smallest_dimensions(dim_1:(u32, u32), dim_2:(u32, u32))->(u32, u32){

    let pix_1 = dim_1.0 * dim_1.1; // tupele values are accessed using dot notation from  zero base indexing
    let pix_2 = dim_2.0 * dim_2.1;
    if pix_1 < pix2 {dim_1} else {dim_2};
}

// standardizing the larger image to match the smaller image

fn standardize_size(image_1: DynamicImage, image_2: DynamicImage)->(DynamicImage, DynamicImage){

    let(width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());

    println!("width: {}, height: {}\n", width, height);

    if(image_2.dimensions() == (width, height)){
        (image_1.resize_exact(image_2, width, height), Triangle)
    }else{
        (image_2.resize_exact(image_1, width, height), Triangle)
    }
}
// the resize_exact() method is used to resize the image by implementing on the DynamicImage struct 
//by mutably borrowing the image_1 and image_2 variables - width, height and FilterType.