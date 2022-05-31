pub const PI: f32 = 3.1415926535897932385;

pub fn deg2rad(deg: f32) -> f32 {
    deg * PI / 180.0
}

pub fn clamp (x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
