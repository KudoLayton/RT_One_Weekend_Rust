extern crate image;

use std::io::Write;

mod vec3;
mod color;

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
            let pixel_color: color::Color = vec3::Vec3::new(
                (i as f64) / ((image_width - 1) as f64),
                (j as f64) / ((image_height - 1) as f64),
                0.25
            );

            color::Color::write_color(&mut image_buffer, pixel_color);
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
