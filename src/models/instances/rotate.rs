use std::{ops::Range, rc::Rc};

use glam::{DVec3, DVec3 as Point};

use crate::{
    models::{
        objects::global::{HitRecord, Hittable},
        ray::Ray,
    },
    utils::math::degrees_to_radians,
};

pub struct Rotate {
    pub object: Rc<dyn Hittable>,
    pub cos_theta: f64,
    pub sin_theta: f64,
}

impl Hittable for Rotate {
    fn hit(&self, ray: Ray, ray_t: Range<f64>, rec: &mut HitRecord) -> bool {
        // Transform the ray from 'world' space to 'object' space
        let origin: Point = Point::new(
            (self.cos_theta * ray.origin.x) - (self.sin_theta * ray.origin.z),
            ray.origin.y,
            (self.sin_theta * ray.origin.x) + (self.cos_theta * ray.origin.z),
        );

        let direction: DVec3 = DVec3::new(
            (self.cos_theta * ray.direction.x) - (self.sin_theta * ray.direction.z),
            ray.direction.y,
            (self.sin_theta * ray.direction.x) + (self.cos_theta * ray.direction.z),
        );

        let rotated_ray: Ray = Ray::new(origin, direction);

        if !self.object.hit(rotated_ray, ray_t, rec) {
            return false;
        }

        rec.point = Point::new(
            (self.cos_theta * rec.point.x) + (self.sin_theta * rec.point.z),
            rec.point.y,
            (-self.sin_theta * rec.point.x) + (self.cos_theta * rec.point.z),
        );

        rec.normal = DVec3::new(
            (self.cos_theta * rec.normal.x) + (self.sin_theta * rec.normal.z),
            rec.normal.y,
            (-self.sin_theta * rec.normal.x) + (self.cos_theta * rec.normal.z),
        );

        true
    }
}

impl Rotate {
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> Self {
        let radians: f64 = degrees_to_radians(angle);
        let sin_theta: f64 = radians.sin();
        let cos_theta: f64 = radians.cos();

        Self {
            object,
            cos_theta,
            sin_theta,
        }
    }
}
