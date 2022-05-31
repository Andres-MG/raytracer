use std::rc::Rc;
use rand::{Rng, thread_rng};
use std::io::Write;
use std::fs::File;
use raytracer::prelude::*;

fn ray_color(r: Ray, world: &Scene, depth: usize) -> Color {
    // Do not go over depth with children
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    // Look for a hit otherwise
    match world.hit(&r, 0.001, 100.0) {

        // Hit found
        Some(rec) => {
            let mut attenuation = Color::new(0.0, 0.0, 0.0);
            match rec.mat.scatter(&r, &rec, &mut attenuation) {
                Some(sr) => {
                    return ray_color(sr, world, depth - 1) * attenuation;
                },
                None => {
                    return attenuation;
                },
            }
        },

        // Shadow rays should go here
        None => {
            let t = 0.5 * (r.direction.y + 1.0);
            return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
        },
    };
}

#[allow(dead_code)]
fn test_scene() -> Scene {
    let mat_left = Rc::new(Dielectric::new(1.5));
    let mat_centre = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.05));
    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));

    Scene {objects: vec![
        // Centre
        Rc::new(Sphere {
            centre: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            mat: Rc::clone(&mat_centre),
        }),
        // Left
        Rc::new(Sphere {
            centre: Point3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            mat: Rc::clone(&mat_left),
        }),
        Rc::new(Sphere {
            centre: Point3::new(-1.0, 0.0, -1.0),
            radius: -0.4,
            mat: Rc::clone(&mat_left),
        }),
        // Right
        Rc::new(Sphere {
            centre: Point3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            mat: Rc::clone(&mat_right),
        }),
        // Ground
        Rc::new(Sphere {
            centre: Point3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            mat: Rc::clone(&mat_ground),
        }),
    ]}
}

#[allow(dead_code)]
fn random_scene() -> Scene {
    // Ground
    let material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let mut world = Scene {objects: vec![
        Rc::new(Sphere {
            centre: Point3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            mat: Rc::clone(&material),
        })
    ]};

    // Random small spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = thread_rng().gen::<f32>();
            let centre = Point3::new(
                a as f32 + thread_rng().gen::<f32>() * 0.9,
                0.2,
                b as f32 + thread_rng().gen::<f32>() * 0.9,
            );

            if (centre - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::unit_random() * Color::unit_random();
                    let mat_sphere = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere {
                        centre,
                        radius: 0.2,
                        mat: Rc::clone(&mat_sphere),
                    }));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::unit_random() * Color::unit_random();
                    let fuzz = thread_rng().gen_range(0.0..0.5);
                    let mat_sphere = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere {
                        centre,
                        radius: 0.2,
                        mat: Rc::clone(&mat_sphere),
                    }));
                } else {
                    // Glass
                    let mat_sphere = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere {
                        centre,
                        radius: 0.2,
                        mat: Rc::clone(&mat_sphere),
                    }));
                }
            }
        }
    }

    // Left sphere
    let material = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere {
        centre: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: Rc::clone(&material),
    }));

    // Centre sphere
    let material = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere {
        centre: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: Rc::clone(&material),
    }));

    // Right sphere
    let material = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere {
        centre: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: Rc::clone(&material),
    }));

    return world;
}

fn main() -> std::io::Result<()> {

    // Image & camera
    let aspect_ratio = 3.0 / 2.0;
    let image_width: usize = 1200;
    let samples_per_pixel: usize = 500;
    let max_depth: usize = 50;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = 10.0;

    let image_height: usize = (image_width as f32 / aspect_ratio) as usize;
    let mut image = vec![[0, 0, 0]; image_height * image_width];

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        30.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // World
    // let world = test_scene();
    let world = random_scene();

    // Render
    let mut ind = 0;
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (i as f32 + thread_rng().gen::<f32>()) / (image_width - 1) as f32;
                let v = (j as f32 + thread_rng().gen::<f32>()) / (image_height - 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_color += &ray_color(r, &world, max_depth);
            }
            image[ind] = postprocess_color(pixel_color, samples_per_pixel);
            ind += 1;
        }
    }

    let mut fh = File::create("test.ppm").expect("Unable to create the file 'test.ppm'.");
    writeln!(fh, "P3\n{} {}\n255", image_width, image_height)?;
    for pixel in image {
        writeln!(fh, "{} {} {}", pixel[0], pixel[1], pixel[2])?;
    }

    Ok(())
}
