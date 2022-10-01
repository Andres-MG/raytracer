pub mod utilities;
pub mod vector;
mod ray;
mod objects;
mod camera;
mod scenes;

use std::error::Error;
use rand::{Rng, thread_rng};
use std::sync::{Arc, Mutex};
use rayon::{self, prelude::*};

use camera::Camera;
use ray::{ray_color, postprocess_color};
use vector::{Point3, Color};
use objects::Scene;

pub struct Config {
    pub aspect_ratio: f32,
    pub image_width: usize,
    pub image_height: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Point3,
    pub aperture: f32,
    pub dist_to_focus: f32,
    pub num_threads: usize,
    pub batch_size: usize,
}

pub fn run(conf: Config) -> Result<Vec<[i32; 3]>, Box<dyn Error>> {

    let image = Arc::new(Mutex::new(vec![[0, 0, 0]; conf.image_height * conf.image_width]));

    let cam = Camera::new(
        conf.lookfrom,
        conf.lookat,
        conf.vup,
        30.0,
        conf.aspect_ratio,
        conf.aperture,
        conf.dist_to_focus,
    );

    // World
    // let world = scenes::test_scene();
    let world = scenes::random_scene();

    // Render
    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(conf.num_threads)
        .build()
        .unwrap();

    let full_loops = conf.image_width * conf.image_height / conf.batch_size;
    let final_loop = conf.image_width * conf.image_height % conf.batch_size;

    thread_pool.install(|| {
        (0..full_loops).into_par_iter().for_each(|i| {
            let start = i * conf.batch_size;
            let end = start + conf.batch_size;
            let image_ref = Arc::clone(&image);
            render_task(
                image_ref,
                start,
                end,
                conf.image_width,
                conf.image_height,
                conf.samples_per_pixel,
                conf.max_depth,
                &cam,
                &world,
            )
        });
    });

    if final_loop > 0 {
        let start = full_loops * conf.batch_size;
        let end = conf.image_width * conf.image_height;
        let image_ref = image.clone();
        render_task(
            image_ref,
            start,
            end,
            conf.image_width,
            conf.image_height,
            conf.samples_per_pixel,
            conf.max_depth,
            &cam,
            &world,
        );
    }

    let image = image.lock().unwrap().to_vec();
    Ok(image)
}

fn render_task(
    image: Arc<Mutex<Vec<[i32; 3]>>>,
    start: usize,
    end: usize,
    image_width: usize,
    image_height: usize,
    samples_per_pixel: usize,
    max_depth: usize,
    cam: &Camera,
    world: &Scene,
) {
    for ilocal in start..end {
        let i = ilocal % image_width;
        let j = image_height - 1 - (ilocal / image_width);
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        for _ in 0..samples_per_pixel {
            let u = (i as f32 + thread_rng().gen::<f32>()) / (image_width - 1) as f32;
            let v = (j as f32 + thread_rng().gen::<f32>()) / (image_height - 1) as f32;
            let r = cam.get_ray(u, v);
            pixel_color += &ray_color(r, &world, max_depth);
        }
        let mut local_image = image.lock().unwrap();
        local_image[ilocal] = postprocess_color(pixel_color, samples_per_pixel);
    }
}

