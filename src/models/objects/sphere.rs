use std::{ops::Range, rc::Rc};

use crate::models::{materials::material::Material, ray::Ray};
use glam::{DVec3 as Point, DVec3};

use super::global::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    mat:Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, mat:Rc<dyn Material>) -> Self {
        Self { center, radius, mat, }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_t:Range<f64>, rec: &mut HitRecord) -> bool {
        let oc: DVec3 = self.center - ray.origin;
        let a: f64 = ray.direction.length_squared();
        let half_b: f64 = ray.direction.dot(oc);
        let c: f64 = oc.length_squared() - self.radius * self.radius;

        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd: f64 = discriminant.sqrt();

        let mut root: f64 = (half_b - sqrtd) / a;
        if !ray_t.contains(&root) {
            root = (half_b + sqrtd) / a;
            if !ray_t.contains(&root) {
                return false;
            }
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        rec.normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray);
        rec.mat = self.mat.clone();
        true
    }
}

