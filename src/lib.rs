mod utilities;
mod vector;
mod ray;
mod objects;
mod camera;

pub mod prelude {
    pub use crate::utilities::*;
    pub use crate::vector::*;
    pub use crate::ray::*;
    pub use crate::objects::*;
    pub use crate::camera::*;
}
