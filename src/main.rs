extern crate image;

mod vec3;
mod color;
mod ray;

use std::io::Write;
use vec3::{Vec3, Point3};
use color::Color;
use ray::Ray;

fn ray_color(r: ray::Ray) -> Color {
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return  Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7,  1.0) * t;
}

fn main() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as usize;
    let mut image_buffer = Vec::<u8>::with_capacity(image_width * image_height * 3);

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin.clone() - horizontal.clone() / 2.0 - vertical.clone() / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render

    let origin_ref = &origin;
    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {} ", j);
        std::io::stdout().flush().unwrap();
        for i in 0..image_width {
            let u = (i as f64) / ((image_width - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64);

            let r = Ray::new(origin_ref.clone(), lower_left_corner.clone() + horizontal.clone() * u + vertical.clone() * v - origin.clone());
            let pixel_color = ray_color(r);

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
