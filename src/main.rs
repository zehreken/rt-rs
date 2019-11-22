extern crate image;
extern crate rand;
use rand::Rng;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::*;
use std::time::{Duration, Instant};
mod primitives;
use crate::primitives::vec3::*;
mod ray;
use crate::ray::ray::*;
mod sphere;
use crate::sphere::sphere::*;
mod camera;
use crate::camera::camera::*;
mod fps_utils;
mod utility;
use crate::fps_utils::fps_utils::*;

pub const WIDTH: u32 = 200;
pub const HEIGHT: u32 = 150;
pub const SAMPLE: u32 = 10;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rt_rs", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let mut framebuffer = texture_creator
        .create_texture(PixelFormatEnum::RGB24, TextureAccess::Static, WIDTH, HEIGHT)
        .unwrap();

    const CHANNEL_COUNT: usize = 3;
    let mut pixels: [u8; WIDTH as usize * HEIGHT as usize * CHANNEL_COUNT] =
        [255; WIDTH as usize * HEIGHT as usize * CHANNEL_COUNT];
    let mut offset: usize = 0;
    framebuffer
        .update(None, &pixels, WIDTH as usize * CHANNEL_COUNT)
        .unwrap();
    let mut colors: [Vec3; WIDTH as usize * HEIGHT as usize] =
        [Vec3::zero(); WIDTH as usize * HEIGHT as usize];

    let mut rng = rand::thread_rng();

    let camera = Camera::get_camera(WIDTH, HEIGHT);
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

    // objects.push(Sphere::new(Vec3::new(0.0, -1000.5, -1.0), 1000.0)); This causes a weird glitch

    let mut event_pump = sdl_context.event_pump().unwrap();

    // for y in 0..HEIGHT {
    //     for x in 0..WIDTH {
    //         let index: usize = ((x + y * WIDTH as u32) * CHANNEL_COUNT as u32) as usize;
    //         let u: f32 = (x as f32 + rng.gen::<f32>()) / WIDTH as f32;
    //         let v: f32 = ((HEIGHT - y) as f32 + rng.gen::<f32>()) / HEIGHT as f32; // invert y
    //         let ray = camera.get_ray(u, v);
    //         colors[index] = colors[index] + color(ray, &objects, 0);

    //         pixels[index] = (colors[index].r() * 255.0) as u8; //pixels[index] + (col.r() * 255.0) as u8) / 2;
    //         pixels[index + 1] = (colors[index].g() * 255.0) as u8; //(pixels[index + 1] + (col.g() * 255.0) as u8) / 2;
    //         pixels[index + 1] = (colors[index].b() * 255.0) as u8; //(pixels[index + 2] + (col.b() * 255.0) as u8) / 2;
    //     }
    // }

    let mut fps_counter = FpsCounter::new();
    let mut now = Instant::now();
    let mut fps_counts = Vec::new();

    let mut sample_count: f32 = 1.0; // This is a divider, can't be zero
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color_index = (x + y * WIDTH as u32) as usize;
                let index: usize = ((x + y * WIDTH as u32) * CHANNEL_COUNT as u32) as usize;
                let u: f32 = (x as f32 + rng.gen::<f32>()) / WIDTH as f32;
                let v: f32 = ((HEIGHT - y) as f32 + rng.gen::<f32>()) / HEIGHT as f32; // invert y
                let ray = camera.get_ray(u, v);
                colors[color_index] = colors[color_index] + color(ray, &objects, 0);

                let r = (colors[color_index].r() / sample_count).sqrt();
                let g = (colors[color_index].g() / sample_count).sqrt();
                let b = (colors[color_index].b() / sample_count).sqrt();
                pixels[index] = (r * 255.0) as u8;
                pixels[index + 1] = (g * 255.0) as u8;
                pixels[index + 2] = (b * 255.0) as u8;
            }
        }

        sample_count += 1.0;

        // let state = event_pump.mouse_state();
        // println!("mouse x: {}, y: {}", state.x(), state.y());
        // let point: usize = ((state.x() + state.y() * WIDTH as i32) * CHANNEL_COUNT as i32) as usize;
        // pixels[point] = 0;
        // pixels[point + 1] = 0;
        // pixels[point + 2] = 0;
        framebuffer
            .update(None, &pixels, WIDTH as usize * CHANNEL_COUNT)
            .unwrap();

        canvas
            .copy(&framebuffer, None, Rect::new(0, 0, WIDTH, HEIGHT))
            .unwrap();

        canvas.present();
        canvas.clear();

        let duration: Duration = Instant::now() - now;
        let fps_count = fps_counter.tick(duration.as_millis());
        if (fps_count > 0) {
            fps_counts.push(fps_count);
        }
        now = Instant::now();

        std::thread::sleep(Duration::from_millis(20));
    }

    let mut fps_sum = 0;
    for i in &fps_counts {
        fps_sum += i;
    }
    let average_fps = fps_sum as f32 / fps_counts.len() as f32;
    println!("Average fps: {}", average_fps);

    return;
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

        // println!("{} / {}", x * HEIGHT, WIDTH * HEIGHT);
    }
    img_buf.save("out/basic.png").unwrap();
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
