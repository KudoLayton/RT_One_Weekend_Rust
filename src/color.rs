use crate::vec3::Vec3;

pub type Color = crate::vec3::Vec3;

impl Color {
	pub fn write_color(image_buffer: &mut Vec<u8>, pixel_color: Self){
		image_buffer.push((255.999 * pixel_color.x()) as u8);
		image_buffer.push((255.999 * pixel_color.y()) as u8);
		image_buffer.push((255.999 * pixel_color.z()) as u8);
	}
}