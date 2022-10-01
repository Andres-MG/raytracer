pub mod utilities;
pub mod vector;
mod ray;
mod objects;
mod camera;
mod scenes;

use std::error::Error;
use rand::{Rng, thread_rng};
use camera::Camera;
use ray::{ray_color, postprocess_color};
use vector::{Point3, Color};

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
}

pub fn run(conf: Config) -> Result<Vec<[i32; 3]>, Box<dyn Error>> {

    let mut image = vec![[0, 0, 0]; conf.image_height * conf.image_width];

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
    let mut ind = 0;
    for j in (0..conf.image_height).rev() {
        for i in 0..conf.image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..conf.samples_per_pixel {
                let u = (i as f32 + thread_rng().gen::<f32>()) / (conf.image_width - 1) as f32;
                let v = (j as f32 + thread_rng().gen::<f32>()) / (conf.image_height - 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_color += &ray_color(r, &world, conf.max_depth);
            }
            image[ind] = postprocess_color(pixel_color, conf.samples_per_pixel);
            ind += 1;
        }
    }
    Ok(image)
}

