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

    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);

    let combined_data = combine_images(image_1, image_2);

    output.set_data(combined_data)?;
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
    BufferTooSmall,
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


//////////////////Creating a Floating Image////////////////////////////


// Defining a struct named FloatingImage - temporary struct to hold the metadata of the output image

struct FloatingImage{
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

// To implement the FloatingImage struct we need to implement a new () function

impl FloatingImage{

    fn new(width:u32, height: u32, name: String)-> Self{
        let buffer_capacity = 3_655_744;
        let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
        FloatingImage{
            width,
            height,
            data: buffer,
            name
        }
    }
fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
    // If the previously assigned buffer is too small to hold the new data
    if data.len() > self.data.capacity() {
      return Err(ImageDataErrors::BufferTooSmall);
    }
    self.data = data;
    Ok(())
  }

}


// creating an instance of the FloatingImage struct in the main function.
//using the image_1 variable's width and height values and the name of the output image
//from the third argument strored in args.


////////////////////////////Creating the Combined Image Data//////////////////////

//we need to first process the images to a vector of RGBA pixels as u8s type 
//and then combine them using the alternate_pixels() function    

fn combine_images ( image_1: DynamicImage, image_2:DynamicImage) -> Vec<u8>{
    let vec_1 = image_1.to_rgb8().into_vec();
    let vec_2 = image_2.to_rgb8().into_vec();

    alternate_pixels(vec_1,vec_2)
}


// this function alternates between vec_1 and vec_2, and copies 4 bytes from vec_1 to combined_data - if the index
// is  a multiple of 8, otherwise it copies 4 bytes from vec_2 to combined_data.
fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8> )-> Vec<u8>{

    // A variable called combined data is created. Which is a Vec<u8> with the same length of vec_1
    let mut combined_data = vec![0u8, vec_1.len()];

    let mut i = 0;
    while i < vec_1.len(){
        if i % 8 == 0{
            combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
        } else {
            combined_data.splice(i..= i+3, set_rgba(&vec_2, i, i + 3));
        }

        i += 4;
    }

    combined_data

}

/////////////////The set_rgba function is a helper function which extracts a slice of 4 bytes(representing
//RGBA values) from a given vector.////////////////////////////////

fn set_rgba(vec: &Vec<u8>, start: usize, end:usize) -> Vec<u8>{
    let mut rgba = Vec::new();
    for i in start..=end{
        let  val = match vec.get(i){
            Some(d) => *d,
            None => panic!("Index out of bounds")

        };
        rgba.push(val);
    }
    rgba

}


///////////////////////////Attaching the Combined Data to the Floating////////////////////////////////


//To set the data of combined_data into the output image, a method on FloatingImage struct  is defined to set the 
//data field of output to the value of combined_data.



///////////////