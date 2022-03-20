mod utilities;

use crate::utilities::output::image::Image;
use std::time::Instant;

fn main() {
    // let mut image = Image::from_dimensions(256, 256);
    let mut image = Image::from_ratio(500, 1.77777777);

    let now = Instant::now();
    for (idx, (row, col)) in Image::walk(image.width, image.height).enumerate() {
        let r = (col as f64) / (image.height as f64);
        let g = (row as f64) / (image.width as f64);
        let b = 0.25;

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;

        let pixel = &mut image.buffer[idx];
        pixel.r = ir;
        pixel.g = ig;
        pixel.b = ib;
    }
    println!("Rendered canvas in {}s", now.elapsed().as_secs());

    image.save("/home/css/path-tracer/out", "rainbow");
}
