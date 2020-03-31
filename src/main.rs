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

pub const SAMPLE: u32 = 10;

fn main() {
    let mut fps_counter = FpsCounter::new();
    let mut now = Instant::now();
    let mut fps_counts = Vec::new();

    // tracer::save_image(640, 480, 5);
    // trace_with_minifb(640, 480);
    trace_with_sdl(200, 150);

    let duration: Duration = Instant::now() - now;
    let fps_count = fps_counter.tick(duration.as_millis());
    if fps_count > 0 {
        fps_counts.push(fps_count);
    }
    now = Instant::now();

    let mut fps_sum = 0;
    for i in &fps_counts {
        fps_sum += i;
    }
    let average_fps = fps_sum as f32 / fps_counts.len() as f32;
    println!("Average fps: {}", average_fps);
}

fn trace_with_sdl(width: u32, height: u32) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rt-rs", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut scene = tracer::create_scene(width, height, 3);

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let mut framebuffer = texture_creator
        .create_texture(PixelFormatEnum::RGB24, TextureAccess::Static, width, height)
        .unwrap();

    const CHANNEL_COUNT: usize = 3;
    framebuffer
        .update(None, &scene.pixels, width as usize * CHANNEL_COUNT)
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

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

        framebuffer
            .update(None, &scene.pixels, width as usize * CHANNEL_COUNT)
            .unwrap();

        canvas
            .copy(&framebuffer, None, Rect::new(0, 0, width, height))
            .unwrap();

        canvas.present();
        canvas.clear();

        std::thread::sleep(Duration::from_millis(20));
    }
}

fn trace_with_minifb(width: usize, height: usize) {
    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(
        "rt-rs - ESC to exit",
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
