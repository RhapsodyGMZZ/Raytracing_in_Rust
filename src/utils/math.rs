use std::{f64::consts::PI, ops::Range};

use glam::DVec3;
use rand::Rng;

pub fn random_float() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random(min: f64, max: f64) -> f64 {
    min + (max - min) * random_float()
}

pub fn random_float_vector() -> DVec3 {
    DVec3::new(random_float(), random_float(), random_float())
}
pub fn random_vector(min: f64, max: f64) -> DVec3 {
    DVec3::new(random(min, max), random(min, max), random(min, max))
}
pub fn random_unit_vector() -> DVec3 {
    loop {
        let p: DVec3 = random_vector(-1.0, 1.0);
        let lensq: f64 = p.length_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

pub fn near_zero(vec: DVec3) -> bool {
    let s: f64 = 1.0e-8;
    vec[0].abs() < s && vec[1].abs() < s && vec[2].abs() < s
}

pub fn reflect(v: DVec3, n: DVec3) -> DVec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(uv: DVec3, n: DVec3, etai_over_etat: f64) -> DVec3 {
    let cos_theta: f64 = ((-uv).dot(n)).min(1.0);
    let r_out_perp: DVec3 = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel: DVec3 = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;
    r_out_perp + r_out_parallel
}

pub fn reflectance(cosinus: f64, refraction_index: f64) -> f64 {
    let mut r0: f64 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0.powi(2);
    r0 + (1.0 - r0) * (1.0 - cosinus).powi(5)
}

pub fn degrees_to_radians(degree: f64) -> f64 {
    degree * PI / 180.0
}

pub fn random_in_unit_disk() -> DVec3 {
    loop {
        let p: DVec3 = DVec3::new(random(-1.0, 1.0), random(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn expand(self_range: Range<f64>, delta: f64) -> Range<f64> {
    let padding: f64 = delta / 2.0;
    self_range.start - padding..self_range.end + padding
}

pub fn new_interval(a: Range<f64>, b: Range<f64>) -> Range<f64> {
    let min: f64 = if a.start <= b.start { a.start } else { b.start };
    let max: f64 = if a.end >= b.end { a.end } else { b.end };
    min..max
}

pub fn random_int(min: f64, max: f64) -> i32 {
    return random(min, max+1.0) as i32;
}