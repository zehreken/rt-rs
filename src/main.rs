extern crate image;
mod primitives;
use crate::primitives::vec3::*;
mod ray;
use crate::ray::ray::*;
mod sphere;
use crate::sphere::sphere::*;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 400;
pub const SAMPLE: u32 = 100;

fn main() {
    let _vector = Vec3::zero();
    println!("{}", _vector);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut img_buf = image::ImageBuffer::new(WIDTH, HEIGHT);
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let u: f32 = x as f32 / WIDTH as f32;
        let v: f32 = (HEIGHT - y) as f32 / HEIGHT as f32; // invert y

        let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
        let color = color(ray);
        *pixel = image::Rgb([
            (color.r() * 255.0) as u8,
            (color.g() * 255.0) as u8,
            (color.b() * 255.0) as u8,
        ]);
    }
    img_buf.save("out/basic.png").unwrap();
}

fn color(ray: Ray) -> Vec3 {
    let sphere: Sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let mut t: f32 = sphere.hit(ray, 0.0, 100.0);
    if t > 0.0 {
        let normal: Vec3 = (ray.point_at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * Vec3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    }

    let unit_direction: Vec3 = ray.direction().unit_vector();
    t = 0.5 * (unit_direction.y() + 1.0);

    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}
