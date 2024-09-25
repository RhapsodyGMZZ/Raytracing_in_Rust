use std::{ops::Range, rc::Rc};

use crate::models::{materials::material::{Lambertian, Material}, ray::Ray};
use glam::{DVec3, DVec3 as Point, DVec3 as Color};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Point,
    pub normal: DVec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Point::new(0.0, 0.0, 0.0),
            normal: Point::new(0.0, 0.0, 0.0),
            mat: Rc::new(Lambertian::new(Color::new(0.0, 0.0, 0.0))),
            t: 0.0,
            front_face: false,
            u:0.0,
            v:0.0
        }
    }

    pub fn set_face_normal(&mut self, ray: Ray) {
        self.front_face = ray.direction.dot(self.normal) < 0.0;
        self.normal = if self.front_face {
            self.normal
        } else {
            -1.0 * self.normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, ray_t: Range<f64>, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object.clone());
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, ray_t: Range<f64>, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = ray_t.end;

        for object in self.objects.iter() {
            if object.hit(ray, ray_t.start..closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t.clone();
                *rec = temp_rec.clone()
            }
        }
        hit_anything
    }
}
