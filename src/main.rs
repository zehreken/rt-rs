extern crate image;
extern crate rand;
use rand::Rng;
mod primitives;
use crate::primitives::vec3::*;
mod ray;
use crate::ray::ray::*;
mod sphere;
use crate::sphere::sphere::*;
mod camera;
use crate::camera::camera::*;
mod utility;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 400;
pub const SAMPLE: u32 = 10;

fn main() {
    let mut rng = rand::thread_rng();

    let camera = Camera::new();
    let mut objects: Vec<Sphere> = Vec::new();
    objects.push(Sphere::new(Vec3::new(0.5, 0.0, -1.0), 0.5, 0));
    objects.push(Sphere::new(Vec3::new(-0.5, 0.0, -1.0), 0.5, 0));

    objects.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, 0));
    // objects.push(Sphere::new(Vec3::new(0.0, -1000.5, -1.0), 1000.0)); This causes a weird glitch

    let mut img_buf = image::ImageBuffer::new(WIDTH, HEIGHT);
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let mut col = Vec3::zero();
        for _z in 0..SAMPLE {
            let u: f32 = (x as f32 + rng.gen::<f32>()) / WIDTH as f32;
            let v: f32 = ((HEIGHT - y) as f32 + rng.gen::<f32>()) / HEIGHT as f32; // invert y
            let ray = camera.get_ray(u, v);
            col = col + color(ray, &objects, 0);
        }

        col = col / SAMPLE as f32;
        col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt()); // Gamma correction

        *pixel = image::Rgb([
            (col.r() * 255.0) as u8,
            (col.g() * 255.0) as u8,
            (col.b() * 255.0) as u8,
        ]);
    }
    img_buf.save("out/basic.png").unwrap();
}

fn color(ray: Ray, objects: &[Sphere], depth: u8) -> Vec3 {
    let mut hit_record: HitRecord = HitRecord::new();
    let mut has_hit = false;
    let t_min: f32 = 0.0;
    let mut closest_so_far: f32 = std::f32::MAX;
    let mut temp_obj = Sphere::new(Vec3::zero(), 0.0, 0);

    for obj in objects {
        if obj.hit(ray, t_min, closest_so_far, &mut hit_record) {
            has_hit = true;
            closest_so_far = hit_record.t;
            temp_obj = *obj;
        }
    }

    if has_hit {
        // let target: Vec3 = hit_record.p + hit_record.normal + random_in_unit_sphere();
        // return 0.5
        //     * color(
        //         Ray::new(hit_record.p, target - hit_record.p),
        //         objects,
        //         depth + 1,
        //     );
        let mut reflect_record:ReflectRecord = ReflectRecord::new(Ray::new(Vec3::zero(), Vec3::zero()), Vec3::zero());
        if depth < 50 && temp_obj.scatter(&mut hit_record, &mut reflect_record) {
            return reflect_record.attenuation * color(reflect_record.scattered, objects, depth + 1);
        } else {
            return Vec3::zero();
        }
    } else {
        let unit_direction: Vec3 = ray.direction().unit_vector();
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);

        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}
