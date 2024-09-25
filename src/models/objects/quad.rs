use std::{ops::Range, rc::Rc};

use glam::{DVec3, DVec3 as Point};

use crate::models::{materials::material::Material, ray::Ray};

use super::global::{HitRecord, Hittable, HittableList};

pub struct Quad {
    pub q: Point,
    pub u: DVec3,
    pub v: DVec3,
    pub w: DVec3,
    pub mat: Rc<dyn Material>,
    pub normal: DVec3,
    pub d: f64,
}

impl Quad {
    pub fn new(q: Point, u: DVec3, v: DVec3, mat: Rc<dyn Material>) -> Self {
        let n: DVec3 = u.cross(v);
        let normal: DVec3 = n.normalize();
        let d: f64 = normal.dot(q);
        let w = n / n.length_squared();

        Self {
            q,
            u,
            v,
            w,
            mat,
            d,
            normal,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: Ray, ray_t: Range<f64>, rec: &mut HitRecord) -> bool {
        let denom: f64 = self.normal.dot(ray.direction);

        if denom.abs() < 1e-8 {
            return false;
        }

        let t: f64 = (self.d - self.normal.dot(ray.origin)) / denom;
        if !ray_t.contains(&t) {
            return false;
        }

        let intersection: DVec3 = ray.at(t);
        let planar_hitpt_vector: DVec3 = intersection - self.q;
        let alpha: f64 = self.w.dot(planar_hitpt_vector.cross(self.v));
        let beta: f64 = self.w.dot(self.u.cross(planar_hitpt_vector));

        if !is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.point = intersection;
        rec.mat = self.mat.clone();
        rec.normal = self.normal;
        rec.set_face_normal(ray);
        true
    }
}

fn is_interior(alpha: f64, beta: f64, rec: &mut HitRecord) -> bool {
    let unit_interval: Range<f64> = 0.0..1.0;
    if !unit_interval.contains(&alpha) || !unit_interval.contains(&beta) {
        return false;
    }
    rec.u = alpha;
    rec.v = beta;
    true
}

pub fn box_shape(a: Point, b: Point, mat: Rc<dyn Material>) -> Rc<dyn Hittable> {
    let mut sides: HittableList = HittableList::new();

    let min: Point = Point::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max: Point = Point::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx: DVec3 = DVec3::new(max.x - min.x, 0.0, 0.0);
    let dy: DVec3 = DVec3::new(0.0, max.y - min.y, 0.0);
    let dz: DVec3 = DVec3::new(0.0, 0.0, max.z - min.z);

    sides.add(Rc::new(Quad::new(
        Point::new(min.x, min.y, max.z),
        dx,
        dy,
        mat.clone(),
    ))); // front
    sides.add(Rc::new(Quad::new(
        Point::new(max.x, min.y, max.z),
        -dz,
        dy,
        mat.clone(),
    ))); // right
    sides.add(Rc::new(Quad::new(
        Point::new(max.x, min.y, min.z),
        -dx,
        dy,
        mat.clone(),
    ))); // back
    sides.add(Rc::new(Quad::new(
        Point::new(min.x, min.y, min.z),
        dz,
        dy,
        mat.clone(),
    ))); // left
    sides.add(Rc::new(Quad::new(
        Point::new(min.x, max.y, max.z),
        dx,
        -dz,
        mat.clone(),
    ))); // top
    sides.add(Rc::new(Quad::new(
        Point::new(min.x, min.y, min.z),
        dx,
        dz,
        mat.clone(),
    ))); // bottom

    Rc::new(sides)
}
