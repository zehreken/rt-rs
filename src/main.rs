extern crate image;
mod primitives;
use crate::primitives::vec3::*;
mod ray;
use crate::ray::ray::*;
mod sphere;
use crate::sphere::sphere::*;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 400;

fn main() {
    let _vector = Vec3::zero();
    println!("{}", _vector);

    let mut img_buf = image::ImageBuffer::new(WIDTH, HEIGHT);
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }
    img_buf.save("out/basic.png").unwrap();
}
