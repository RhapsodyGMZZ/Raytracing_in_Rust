use glam::{DVec3, DVec3 as Color, DVec3 as Point};

use crate::{models::{objects::global::HitRecord, ray::Ray}, utils::math::{near_zero, random_unit_vector}};

pub trait Material {
    fn scatter(&self, ray_in: Ray, rec: &HitRecord, color_attenuation: &mut Color, scattered: &mut Ray) -> bool;
    fn color_emitted(&self, _u:f64, _v:f64, _point: Point) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub struct Lambertian {
    albedo:Color,
}

impl Lambertian {
    pub fn new(color:Color) -> Self {
        Self{
            albedo:color,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: Ray, rec: &HitRecord, color_attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction: DVec3 = rec.normal + random_unit_vector();
        
        if near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.point, scatter_direction);
        *color_attenuation =  self.albedo;
        true
    }
}