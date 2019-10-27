extern crate image;
mod primitives;
use crate::primitives::vec3::*;
mod ray;
use crate::ray::ray::*;
mod sphere;
use crate::sphere::sphere::*;
mod camera;
use crate::camera::camera::*;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 400;
pub const SAMPLE: u32 = 100;

fn main() {
    let _vector = Vec3::zero();
    println!("{}", _vector);

    let camera = Camera::new();
    let mut objects: Vec<Sphere> = Vec::new();
    objects.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));

    objects.push(Sphere::new(Vec3::new(0.0, -1000.5, -1.0), 1000.0));

    let mut img_buf = image::ImageBuffer::new(WIDTH, HEIGHT);
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let u: f32 = x as f32 / WIDTH as f32;
        let v: f32 = (HEIGHT - y) as f32 / HEIGHT as f32; // invert y

        // let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
        let ray = camera.get_ray(u, v);
        let color = color(ray, &objects);
        *pixel = image::Rgb([
            (color.r() * 255.0) as u8,
            (color.g() * 255.0) as u8,
            (color.b() * 255.0) as u8,
        ]);
    }
    img_buf.save("out/basic.png").unwrap();
}

fn color(ray: Ray, objects: &[Sphere]) -> Vec3 {
    let mut hit_record: HitRecord = HitRecord::new();
    let mut has_hit = false;
    let t_min: f32 = 0.0;
    let mut closest_so_far: f32 = 10000000.0;

    for obj in objects {
        if obj.hit(ray, t_min, closest_so_far, &mut hit_record) {
            has_hit = true;
            closest_so_far = hit_record.t;
        }
    }

    if has_hit {
        return 0.5
            * Vec3::new(
                hit_record.normal.x() + 1.0,
                hit_record.normal.y() + 1.0,
                hit_record.normal.z() + 1.0,
            );
    } else {
        let unit_direction: Vec3 = ray.direction().unit_vector();
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);

        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

// fn color(ray: Ray, world: &HitableList) -> Vec3 {
//     let mut hit_record: HitRecord = HitRecord::new();
//     if world.hit(ray, 0.0, 100.0, &mut hit_record) {
//         return 0.5
//             * Vec3::new(
//                 hit_record.normal.x() + 1.0,
//                 hit_record.normal.y() + 1.0,
//                 hit_record.normal.z() + 1.0,
//             );
//     } else {
//         let unit_direction: Vec3 = ray.direction().unit_vector();
//         let t: f32 = 0.5 * (unit_direction.y() + 1.0);

//         return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
//     }
// }
