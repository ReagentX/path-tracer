mod utilities;

use crate::utilities::output::image::Image;

fn main() {
    let mut image = Image::from_dimensions(10, 10);
    image.save("/home/css/path-tracer/out", "black");
}

