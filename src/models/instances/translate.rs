use std::{ops::Range, rc::Rc};

use glam::DVec3;

use crate::models::{objects::global::{HitRecord, Hittable}, ray::Ray};

pub struct Translate {
    pub object: Rc<dyn Hittable>,
    pub offset: DVec3,
}

impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: DVec3) -> Self {
        Self { object, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: Ray, ray_t: Range<f64>, rec: &mut HitRecord) -> bool {
        let offset_ray: Ray = Ray::new(ray.origin - self.offset, ray.direction);
        
        if !self.object.hit(offset_ray, ray_t, rec) {
            return false
        }

        rec.point += self.offset;
        true
    }
}