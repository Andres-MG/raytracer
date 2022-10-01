use std::sync::Arc;
use rand::{Rng, thread_rng};
use crate::vector::*;
use crate::ray::*;

pub struct HitRecord {
    pub p: Point3,
    pub n: Vec3,
    pub t : f32,
    pub front: bool,
    pub mat: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(p: Point3, n: Vec3, t: f32, mat: Arc<dyn Material>) -> Self {
        Self {p, n, t, front: true, mat}
    }

    pub fn set_face_normal(&mut self, r: &Ray) {
        self.front = r.direction.dot(&self.n) < 0.0;
        self.n = if self.front {self.n} else {-self.n};
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Scene {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl Scene {
    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.objects.push(Arc::clone(&object));
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_rec: Option<HitRecord> = None;
        for object in self.objects.iter() {
            match object.hit(r, t_min, t_max) {
                Some(rec) => {
                    if rec.t < closest_so_far {
                        closest_so_far = rec.t;
                        hit_rec = Some(rec);
                    }
                },
                None => { },
            };
        }
        hit_rec
    }
}

pub struct Sphere<T: Material> {
    pub centre: Point3,
    pub radius: f32,
    pub mat: Arc<T>,
}

impl<T: Material + 'static> Hittable for Sphere<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.centre;
        let a = r.direction.length_squared();
        let h = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let disc = h * h - a * c;

        // Stop if we did not hit the sphere
        if disc < 0.0 {
            return None;

        // Check if the hit is in the given range
        } else {
            let sqrtd = disc.sqrt();
            let mut root = -(h + sqrtd) / a;
            if root < t_min || root > t_max {
                root = (-h + sqrtd) / a;
                if root < t_min || root > t_max {
                    return None;
                }
            }

            // We have a hit
            let t = root;
            let p = r.at(t);
            let outward_normal = (p - self.centre) / self.radius;
            let mut rec = HitRecord::new(
                p,
                outward_normal,
                t,
                Arc::clone(&self.mat) as Arc<dyn Material>
            );
            rec.set_face_normal(r);
            return Some(rec);
        }
    }
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color) -> Option<Ray> {
        *attenuation = self.albedo;
        let mut scatter_dir = rec.n + Vec3::unit_random();
        if scatter_dir.near_zero() {
            scatter_dir = rec.n;
        }
        Some(Ray::new(rec.p, scatter_dir))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzzy: f32) -> Self {
        let fuzz = if fuzzy >= 0.0 && fuzzy < 1.0 { fuzzy } else {1.0};
        Metal {albedo, fuzz}
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3) -> Option<Ray> {
        *attenuation = self.albedo;
        let reflected = reflect(&r_in.direction.normalize(), &rec.n);
        let scattered = Ray::new(rec.p, reflected + Vec3::unit_random() * self.fuzz);
        if scattered.direction.dot(&rec.n) > 0.0 {
            Some(scattered)
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub index: f32,
}

impl Dielectric {
    pub fn new(index: f32) -> Self {
        Dielectric {index}
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // Schlick's approximation
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3) -> Option<Ray> {
        // Attenuation color
        *attenuation = Color::new(1.0, 1.0, 1.0);

        // Scattered ray
        let relative_index = if rec.front { 1.0 / self.index } else { self.index };
        let unit_dir = r_in.direction.normalize();
        let cos_theta = -unit_dir.dot(&rec.n);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        // Ray can be refracted or reflected
        let can_refract = relative_index * sin_theta < 1.0;
        let rand_refract = reflectance(cos_theta, relative_index) < thread_rng().gen();
        let direction = if can_refract && rand_refract {
            refract(&unit_dir, &rec.n, relative_index)
        } else {
            reflect(&unit_dir, &rec.n)
        };
        Some(Ray::new(rec.p, direction))
    }
}
