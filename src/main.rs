pub mod ray;
pub mod hittable;
pub mod vec_math;
pub mod hittable_list;
pub mod camera;
mod material;

use std::{thread::{self, JoinHandle}, sync::{Arc, Mutex}};

use camera::Camera;
use hittable::Sphere;
use hittable_list::HittableList;
use material::Material;
use notan::draw::*;
use notan::math::Vec3;
use notan::prelude::*;
use vec_math::{clamp, rand_f32};

use crate::ray::ray_color;

const IMAGE_WIDTH: usize = 1920 / 1;
const IMAGE_HEIGHT: usize = 1080 / 1;
const NUM_OF_THREADS: usize = 8;

#[derive(AppState)]
struct State {
    texture: Texture,
    bytes: Vec<u8>,
    pixels_array: Arc<Mutex<Box<[[Vec3; IMAGE_HEIGHT]; IMAGE_WIDTH]>>>,
    sample: i32,
    camera: Camera,
    world: Arc<HittableList>,
    update_texture: bool,
    thread_handles: Vec<JoinHandle<()>>,
    allow_new_threads: bool
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(init)
        .add_config(WindowConfig::new().size(1280, 720).vsync(false).title("arm's raytracer").resizable(true))
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn init(_app: &mut App, gfx: &mut Graphics) -> State {
    // Image
    let bytes = vec![0; IMAGE_WIDTH * IMAGE_HEIGHT * 4];

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Vec3::new(0., -1000., -1.2), 1000., Material::Metal(Vec3::new(0.4, 0.4, 0.4), 1.)));

    for i in -9..=9 {
        for j in -9..=9 {
            let radius = rand_f32(0.3, 0.6);
            let mat = if fastrand::f32() < 0.65 {
                Material::Lambertian(Vec3::new(fastrand::f32(), fastrand::f32(), fastrand::f32()))
            } else {
                Material::Metal(Vec3::new(fastrand::f32(), fastrand::f32(), fastrand::f32()), rand_f32(0.0, 1.))
            };

            world.add(
                Sphere::new(
                    Vec3::new(i as f32 * 2.3 + 0.9 * fastrand::f32(), radius, j as f32 * 2.3 + 0.9 * fastrand::f32() - 1.2),
                    radius, 
                    mat
                )
            );
        }
    }
    world.add(Sphere::new(Vec3::new(0., 2., -1.2), 2., Material::Metal(Vec3::new(0.8, 0.8, 0.8), 0.0)));
    world.add(Sphere::new(Vec3::new(8., 4., -1.2), 4., Material::Metal(Vec3::new(0.8, 0.8, 0.8), 0.0)));

    // Camera
    let camera = Camera::new(Vec3::new(-3., 7.5, 5.), Vec3::new(0., 2., -1.2), Vec3::new(0., 1., 0.), 100., IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32);

    // Render
    let pixels_array = create_pixel_array();
    
    let texture = gfx
                    .create_texture()
                    .from_bytes(&bytes, IMAGE_WIDTH as i32, IMAGE_HEIGHT as i32)
                    .with_filter(TextureFilter::Linear, TextureFilter::Linear)
                    .build()
                    .unwrap();
        
    State {
        texture,
        bytes,
        pixels_array: Arc::new(Mutex::new(pixels_array)),
        sample: 0,
        camera,
        world: Arc::new(world),
        update_texture: true,
        thread_handles: vec![],
        allow_new_threads: true
    }
}

fn create_pixel_array() -> Box<[[Vec3; IMAGE_HEIGHT]; IMAGE_WIDTH]> {
    let mut data = std::mem::ManuallyDrop::new(vec![Vec3::default(); IMAGE_WIDTH * IMAGE_HEIGHT]);
    
    unsafe {
        Box::from_raw(data.as_mut_ptr() as *mut [[Vec3; IMAGE_HEIGHT]; IMAGE_WIDTH])
    }
}

fn update(_app: &mut App, state: &mut State) {

    // Spawn threads and perform calculations
    if state.allow_new_threads {
        state.allow_new_threads = false;
        println!("sample: {}", state.sample + 1);
        state.sample += 1;

        for t in 0..NUM_OF_THREADS {
            let camera_clone = state.camera.clone();
            let world_clone = Arc::clone(&state.world);
            let pixels_array_clone = Arc::clone(&state.pixels_array);

            let handle = thread::spawn(move || {
                for i in t * IMAGE_WIDTH / NUM_OF_THREADS..(t * IMAGE_WIDTH / NUM_OF_THREADS + IMAGE_WIDTH / NUM_OF_THREADS) {
                    for j in 0..IMAGE_HEIGHT {
                        if i < IMAGE_WIDTH && j < IMAGE_HEIGHT {
                            let u = (i as f32 + fastrand::f32()) / IMAGE_WIDTH as f32;
                            let v = (j as f32 + fastrand::f32()) / IMAGE_HEIGHT as f32;

                            let r = camera_clone.get_ray(u, v);
                            let pixel_color = ray_color(&r, &world_clone, 50);

                            add_to_pixel(&mut pixels_array_clone.lock().unwrap(), i, IMAGE_HEIGHT - j - 1, pixel_color);
                        }
                    }
                }
            });
            state.thread_handles.push(handle);
        }
    }

    if handles_finished(&state.thread_handles) {
        for _ in 0..NUM_OF_THREADS {
            state.thread_handles.pop().unwrap().join().unwrap();
        }
        state.allow_new_threads = true;
    }

    if state.sample % 10 == 0 || state.sample == 1 {
        state.update_texture = true;
    }
    
    if state.update_texture {
        write_to_bytes(&mut state.pixels_array.lock().unwrap(), &mut state.bytes, state.sample);
    }
}

fn handles_finished(handles: &Vec<JoinHandle<()>>) -> bool {
    for handle in handles {
        if !handle.is_finished() {
            return false;
        }
    }
    true
}

pub fn write_to_bytes(pixels_array: &Box<[[Vec3; IMAGE_HEIGHT]; IMAGE_WIDTH]>, bytes: &mut Vec<u8>, sample: i32) {
    for i in 0..IMAGE_WIDTH {
        for j in 0..IMAGE_HEIGHT {
            let color = pixels_array[i][j];

            let mut r = color.x;
            let mut g = color.y;
            let mut b = color.z;

            let scale = 1. / sample as f32;
            r = (scale * r).sqrt();
            g = (scale * g).sqrt();
            b = (scale * b).sqrt();
    
            let r_u8 = (256. * clamp(r, 0., 0.999)) as u8;
            let g_u8 = (256. * clamp(g, 0., 0.999)) as u8;
            let b_u8 = (256. * clamp(b, 0., 0.999)) as u8;
            
            bytes[(j * IMAGE_WIDTH as usize + i) * 4..(j * IMAGE_WIDTH as usize + i) * 4 + 4].copy_from_slice(&[r_u8, g_u8, b_u8, 255]);
        }
    }
}

fn add_to_pixel(pixels_array: &mut Box<[[Vec3; IMAGE_HEIGHT]; IMAGE_WIDTH]>, i: usize, j: usize, color: Vec3) {
    pixels_array[i][j] += color;
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    if state.update_texture {
        gfx.update_texture(&mut state.texture)
            .with_data(&state.bytes)
            .update()
            .unwrap();
    
        let mut draw = gfx.create_draw();
        draw.image(&state.texture).flip_y(false).position(0., 0.).size(app.window().width() as f32, app.window().height() as f32);

        gfx.render(&draw);

        state.update_texture = false;
    }

    if app.keyboard.was_pressed(KeyCode::Space) {
        state.texture.to_file(gfx, "render.png").unwrap();
        println!("Image Saved");
    }
}