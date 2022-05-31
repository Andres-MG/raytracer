use crate::utilities::clamp;
use crate::vector::{Vec3, Point3, Color};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {origin, direction}
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    (*v) - (*n) * v.dot(n) * 2.0
}

pub fn refract(uv: &Vec3, n: &Vec3, relative_index: f32) -> Vec3 {
    let cos_theta = -uv.dot(n);
    let r_out_perp = ((*uv) + (*n) * cos_theta) * relative_index;
    let r_out_par = -(*n) * f32::sqrt(1.0 - r_out_perp.length_squared());
    r_out_perp + r_out_par
}

pub fn postprocess_color(pixel_color: Color, samples: usize) -> [i32; 3] {
    // Divide by the number of samples and gamma-correct
    let scale = 1.0 / samples as f32;
    let r = (pixel_color.x * scale).sqrt();
    let g = (pixel_color.y * scale).sqrt();
    let b = (pixel_color.z * scale).sqrt();

    // Return the scaled color
    return [
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32,
    ];
}
