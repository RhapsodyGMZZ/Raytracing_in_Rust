use std::{ops::Range, rc::Rc};

use glam::{DVec3, DVec3 as Point};

use crate::models::{materials::material::Material, ray::Ray};

use super::global::{HitRecord, Hittable};
pub struct Cylinder {
    pub origin: Point,
    pub axis: DVec3,
    pub radius: f64,
    pub height: f64,
    pub mat: Rc<dyn Material>,
}

impl Cylinder {
    pub fn new(
        origin: Point,
        axis: DVec3,
        radius: f64,
        height: f64,
        mat: Rc<dyn Material>,
    ) -> Self {
        Self {
            origin,
            axis,
            radius,
            height,
            mat,
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: Ray, ray_t: Range<f64>, rec: &mut HitRecord) -> bool {
        // let oc: DVec3 = ray.origin - self.origin;
        // let a: f64 = ray.direction.length_squared() - self.axis.dot(ray.direction).powi(2);
        // let b: f64 = oc.dot(ray.direction) - self.axis.dot(oc) * self.axis.dot(ray.direction);
        // let c: f64 = oc.length_squared() - self.axis.dot(oc).powi(2) - self.radius.powi(2);
        // let discriminant: f64 = b.powi(2) - a * c;

        // if discriminant < 0.0 {
        //     return false;
        // }

        // let sqrtd: f64 = discriminant.sqrt();
        // let mut root: f64 = (-b - sqrtd) / a;
        // if !ray_t.contains(&root) {
        //     root = (-b + sqrtd) / a;
        //     if !ray_t.contains(&root) {
        //         return false;
        //     }
        // }

        // // Not making an infinite cylinder >> returning false when the hit point is above the max-height set earlier.
        // let hit_point: DVec3 = ray.at(root);
        // let height: f64 = self.axis.dot(hit_point - self.origin);

        // if height < 0.0 || height > self.height {
        //     return false;
        // }

        // rec.t = root;
        // rec.point = ray.at(rec.t);
        // rec.normal =
        //     (rec.point - self.origin - self.axis * (rec.point - self.origin).dot(self.axis))
        //         .normalize();
        // rec.set_face_normal(ray);
        // rec.mat = self.mat.clone();
        // true
        let oc: DVec3 = ray.origin - self.origin;
        let a: f64 = ray.direction.length_squared() - self.axis.dot(ray.direction).powi(2);
        let b: f64 = oc.dot(ray.direction) - self.axis.dot(oc) * self.axis.dot(ray.direction);
        let c: f64 = oc.length_squared() - self.axis.dot(oc).powi(2) - self.radius.powi(2);
        let discriminant: f64 = b.powi(2) - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd: f64 = discriminant.sqrt();
        let mut root: f64 = (-b - sqrtd) / a;
        if !ray_t.contains(&root) {
            root = (-b + sqrtd) / a;
            if !ray_t.contains(&root) {
                return false;
            }
        }

        // Not making an infinite cylinder >> returning false when the hit point is above the max-height set earlier.
        let hit_point: DVec3 = ray.at(root);
        let height: f64 = self.axis.dot(hit_point - self.origin);

        if height < 0.0 || height > self.height {
            // Check for intersection with the caps
            let t1: f64 = (self.origin - ray.origin).dot(self.axis) / ray.direction.dot(self.axis);
            let t2: f64 = (self.origin + self.axis * self.height - ray.origin).dot(self.axis)
                / ray.direction.dot(self.axis);

            if ray_t.contains(&t1) {
                let p1: DVec3 = ray.at(t1);
                if (p1 - self.origin).length_squared() <= self.radius.powi(2) {
                    rec.t = t1;
                    rec.point = p1;
                    rec.normal = -self.axis;
                    rec.set_face_normal(ray);
                    rec.mat = self.mat.clone();
                    return true;
                }
            }

            if ray_t.contains(&t2) {
                let p2: DVec3 = ray.at(t2);
                if (p2 - (self.origin + self.axis * self.height)).length_squared()
                    <= self.radius.powi(2)
                {
                    rec.t = t2;
                    rec.point = p2;
                    rec.normal = self.axis;
                    rec.set_face_normal(ray);
                    rec.mat = self.mat.clone();
                    return true;
                }
            }

            return false;
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        rec.normal =
            (rec.point - self.origin - self.axis * (rec.point - self.origin).dot(self.axis))
                .normalize();
        rec.set_face_normal(ray);
        rec.mat = self.mat.clone();
        true
    }
}
