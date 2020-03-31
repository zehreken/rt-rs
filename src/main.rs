extern crate image;
extern crate rand;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::*;
use std::time::{Duration, Instant};
mod camera;
mod fps_utils;
mod primitives;
mod ray;
mod sphere;
mod utility;
use crate::fps_utils::fps_utils::*;
mod tracer;
use minifb::{Key, Window, WindowOptions};

pub const WIDTH: u32 = 200;
pub const HEIGHT: u32 = 150;
pub const SAMPLE: u32 = 10;

fn main() {
    // tracer::save_image(640, 480, 5);
    trace_with_minifb(640, 480);
    return;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rt-rs", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut scene = tracer::create_scene(WIDTH, HEIGHT, 3);

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let mut framebuffer = texture_creator
        .create_texture(PixelFormatEnum::RGB24, TextureAccess::Static, WIDTH, HEIGHT)
        .unwrap();

    const CHANNEL_COUNT: usize = 3;
    let mut offset: usize = 0;
    framebuffer
        .update(None, &scene.pixels, WIDTH as usize * CHANNEL_COUNT)
        .unwrap();

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

        tracer::update(&mut scene);
        // camera.translate(Vec3::new(0.0, 0.0, -0.01));

        // for y in 0..HEIGHT {
        //     for x in 0..WIDTH {
        //         let color_index = (x + y * WIDTH as u32) as usize;
        //         let index: usize = ((x + y * WIDTH as u32) * CHANNEL_COUNT as u32) as usize;
        //         let u: f32 = (x as f32 + rng.gen::<f32>()) / WIDTH as f32;
        //         let v: f32 = ((HEIGHT - y) as f32 + rng.gen::<f32>()) / HEIGHT as f32; // invert y
        //         let ray = camera.get_ray(u, v);
        //         colors[color_index] = color(ray, &objects, 0);

        //         let r = (colors[color_index].r() / sample_count).sqrt();
        //         let g = (colors[color_index].g() / sample_count).sqrt();
        //         let b = (colors[color_index].b() / sample_count).sqrt();
        //         pixels[index] = (r * 255.0) as u8;
        //         pixels[index + 1] = (g * 255.0) as u8;
        //         pixels[index + 2] = (b * 255.0) as u8;
        //     }
        // }

        // sample_count += 1.0;

        // let state = event_pump.mouse_state();
        // println!("mouse x: {}, y: {}", state.x(), state.y());
        // let point: usize = ((state.x() + state.y() * WIDTH as i32) * CHANNEL_COUNT as i32) as usize;
        // pixels[point] = 0;
        // pixels[point + 1] = 0;
        // pixels[point + 2] = 0;
        framebuffer
            .update(None, &scene.pixels, WIDTH as usize * CHANNEL_COUNT)
            .unwrap();

        canvas
            .copy(&framebuffer, None, Rect::new(0, 0, WIDTH, HEIGHT))
            .unwrap();

        canvas.present();
        canvas.clear();

        let duration: Duration = Instant::now() - now;
        let fps_count = fps_counter.tick(duration.as_millis());
        if fps_count > 0 {
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
}

fn trace_with_sdl() {}

fn trace_with_minifb(width: usize, height: usize) {
    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(
        "Test - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0xff0055; // write someting more funny here!
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to haneld this in a different way
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
