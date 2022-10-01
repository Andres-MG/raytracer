use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

use clap::Parser;
use raytracer::{Config, run};

fn main() -> Result<(), Box<dyn Error>> {

    let conf = Config::parse();

    let now = Instant::now();
    let image = run(&conf)?;
    println!("Render time: {} ms.", now.elapsed().as_millis());

    let mut fh = File::create("test.ppm").expect("Unable to create the file 'test.ppm'.");
    writeln!(fh, "P3\n{} {}\n255", conf.image_width, conf.image_height)?;
    for pixel in image {
        writeln!(fh, "{} {} {}", pixel[0], pixel[1], pixel[2])?;
    }

    Ok(())
}
