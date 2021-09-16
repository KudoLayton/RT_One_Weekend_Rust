use std::io::Write;

extern crate image;

fn main() {

    // Image

    let image_width = 256;
    let image_height = 256;
    let mut image_buffer = Vec::<u8>::with_capacity(image_width * image_height * 3);

    // Render

    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {} ", j);
        std::io::stdout().flush().unwrap();
        for i in 0..image_width {
            let r = (i as f64) / ((image_width - 1) as f64);
            let g = (j as f64) / ((image_width - 1) as f64);
            let b = 0.25f64;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            image_buffer.push(ir);
            image_buffer.push(ig);
            image_buffer.push(ib);
        }
    }

    println!("\nDone!");

    image::save_buffer(
        "image.png", 
        image_buffer.as_slice(), 
        image_width as u32, 
        image_height as u32, 
        image::ColorType::Rgb8
    ).expect("failed to generate image file");
}
