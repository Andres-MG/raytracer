use std::error::Error;
use std::fs::File;
use std::io::Write;

use raytracer::{Config, run};
use raytracer::vector::Point3;

fn main() -> Result<(), Box<dyn Error>> {

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let image_height = (image_width as f32 / aspect_ratio) as usize;
    let conf = Config {
        aspect_ratio,
        image_width,
        image_height,
        samples_per_pixel: 500,
        max_depth: 50,
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        vup: Point3::new(0.0, 1.0, 0.0),
        aperture: 0.1,
        dist_to_focus: 10.0,
    };

    let image = run(conf)?;

    let mut fh = File::create("test.ppm").expect("Unable to create the file 'test.ppm'.");
    writeln!(fh, "P3\n{} {}\n255", image_width, image_height)?;
    for pixel in image {
        writeln!(fh, "{} {} {}", pixel[0], pixel[1], pixel[2])?;
    }

    Ok(())
}
