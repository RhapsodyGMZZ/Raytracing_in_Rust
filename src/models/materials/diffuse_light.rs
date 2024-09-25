use glam::DVec3 as Color;

use crate::models::{objects::global::HitRecord, ray::Ray};

use super::material::Material;

pub struct DiffuseLight {
    pub color: Color,
}

impl DiffuseLight {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: Ray, _rec: &HitRecord, _color_attenuation: &mut Color, _scattered: &mut Ray) -> bool {
        false
    }
    fn color_emitted(&self, _u:f64, _v:f64, _point: Color) -> Color {
        self.color
    }
}