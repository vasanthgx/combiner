
fn get_nth_arg(n: usize)->String{

    std::env::args().nth(n).unwrap()

}

// defining a public struct named Args which consists of 3 public fields of 
//type String: image_1, image_2 and output
// so that we can access them outside of the args.rs file

pub struct Args{
    pub image_1: String,
    pub image_2: String,
    pub output:String,
}


//  next we create a new Args struct in a function called 'new'

impl Args{
    pub fn new() -> Self{
        Args{
            image_1: get_nth_arg(1),
            image_2:get_nth_arg(2),
            output: get_nth_arg(3),

        }
    }
}