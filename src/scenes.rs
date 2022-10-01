use crate::objects::*;
use crate::vector::{Point3, Color};
use std::sync::Arc;
use rand::{Rng, thread_rng};

#[allow(dead_code)]
pub fn test_scene() -> Scene {
    let mat_left = Arc::new(Dielectric::new(1.5));
    let mat_centre = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.05));
    let mat_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));

    Scene {objects: vec![
        // Centre
        Arc::new(Sphere {
            centre: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            mat: Arc::clone(&mat_centre),
        }),
        // Left
        Arc::new(Sphere {
            centre: Point3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            mat: Arc::clone(&mat_left),
        }),
        Arc::new(Sphere {
            centre: Point3::new(-1.0, 0.0, -1.0),
            radius: -0.4,
            mat: Arc::clone(&mat_left),
        }),
        // Right
        Arc::new(Sphere {
            centre: Point3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            mat: Arc::clone(&mat_right),
        }),
        // Ground
        Arc::new(Sphere {
            centre: Point3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            mat: Arc::clone(&mat_ground),
        }),
    ]}
}

pub fn random_scene() -> Scene {
    // Ground
    let material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let mut world = Scene {objects: vec![
        Arc::new(Sphere {
            centre: Point3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            mat: Arc::clone(&material),
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
                    let mat_sphere = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere {
                        centre,
                        radius: 0.2,
                        mat: Arc::clone(&mat_sphere),
                    }));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::unit_random() * Color::unit_random();
                    let fuzz = thread_rng().gen_range(0.0..0.5);
                    let mat_sphere = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere {
                        centre,
                        radius: 0.2,
                        mat: Arc::clone(&mat_sphere),
                    }));
                } else {
                    // Glass
                    let mat_sphere = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere {
                        centre,
                        radius: 0.2,
                        mat: Arc::clone(&mat_sphere),
                    }));
                }
            }
        }
    }

    // Left sphere
    let material = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere {
        centre: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: Arc::clone(&material),
    }));

    // Centre sphere
    let material = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere {
        centre: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: Arc::clone(&material),
    }));

    // Right sphere
    let material = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere {
        centre: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: Arc::clone(&material),
    }));

    return world;
}

