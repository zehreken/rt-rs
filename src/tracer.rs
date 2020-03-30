use crate::camera::camera::*;
use crate::primitives::vec3::*;
use crate::ray::ray::*;
use crate::sphere::sphere::*;
use rand::Rng;

pub struct Scene {
    camera: Camera,
    objects: Vec<Sphere>,
    width: u32,
    height: u32,
    channel_count: usize,
    colors: Vec<Vec3>,
    pub pixels: Vec<u8>,
}

pub fn create_scene(width: u32, height: u32, channel_count: usize) -> Scene {
    let mut camera = Camera::get_camera(width, height);
    let mut objects: Vec<Sphere> = Vec::new();
    objects.push(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        0, // lambertian
        Vec3::new(0.5, 0.1, 0.1),
        0.0,
    ));
    objects.push(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        1, // metal
        Vec3::new(0.9, 0.9, 0.9),
        0.2,
    ));
    objects.push(Sphere::new(
        Vec3::new(-1.0, -0.0, -1.0),
        0.5,
        2, // dielectric
        Vec3::new(0.1, 0.5, 0.1).sqrt().sqrt().sqrt(),
        0.2,
    ));
    objects.push(Sphere::new(
        Vec3::new(0.0, 0.0, 1.0),
        0.5,
        2,
        Vec3::new(0.5, 0.5, 0.3).sqrt().sqrt().sqrt(),
        0.2,
    ));
    objects.push(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        0,
        Vec3::new(0.1, 0.1, 0.5),
        0.0,
    ));
    // objects.push(Sphere::new(Vec3::new(0.0, -1000.5, -1.0), 1000.0)); // This causes a weird glitch

    Scene {
        camera,
        objects,
        width,
        height,
        channel_count,
        colors: vec![Vec3::zero(); width as usize * height as usize],
        pixels: vec![0; width as usize * height as usize * channel_count],
    }
}

pub fn update(scene: &mut Scene) {
    let mut rng = rand::thread_rng();
    let WIDTH = scene.width;
    let HEIGHT = scene.height;
    let CHANNEL_COUNT = scene.channel_count;
    let sample_count = 1.0;

    scene.camera.translate(Vec3::new(0.0, 0.0, -0.01));
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color_index = (x + y * WIDTH as u32) as usize;
            let index: usize = ((x + y * WIDTH as u32) * CHANNEL_COUNT as u32) as usize;
            let u: f32 = (x as f32 + rng.gen::<f32>()) / WIDTH as f32;
            let v: f32 = ((HEIGHT - y) as f32 + rng.gen::<f32>()) / HEIGHT as f32; // invert y
            let ray = scene.camera.get_ray(u, v);
            scene.colors[color_index] = color(ray, &scene.objects, 0);

            let r = (scene.colors[color_index].r() / sample_count).sqrt();
            let g = (scene.colors[color_index].g() / sample_count).sqrt();
            let b = (scene.colors[color_index].b() / sample_count).sqrt();
            scene.pixels[index] = (r * 255.0) as u8;
            scene.pixels[index + 1] = (g * 255.0) as u8;
            scene.pixels[index + 2] = (b * 255.0) as u8;
        }
    }
}

fn color(ray: Ray, objects: &[Sphere], depth: u8) -> Vec3 {
    let mut hit_record: HitRecord = HitRecord::new();
    let mut has_hit = false;
    let t_min: f32 = 0.0;
    let mut closest_so_far: f32 = std::f32::MAX;
    let mut temp_obj = Sphere::new(Vec3::zero(), 0.0, 0, Vec3::zero(), 0.0);

    for obj in objects {
        if obj.hit(ray, t_min, closest_so_far, &mut hit_record) {
            has_hit = true;
            closest_so_far = hit_record.t;
            temp_obj = *obj;
        }
    }

    if has_hit {
        let mut reflect_record: ReflectRecord =
            ReflectRecord::new(Ray::new(Vec3::zero(), Vec3::zero()), Vec3::zero());
        if depth < 50 && temp_obj.scatter(ray, &mut hit_record, &mut reflect_record) {
            return reflect_record.attenuation
                * color(reflect_record.scattered, objects, depth + 1);
        } else {
            return Vec3::zero();
        }
    } else {
        let unit_direction: Vec3 = ray.direction().unit_vector();
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);

        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}
