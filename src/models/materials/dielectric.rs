use glam::{DVec3, DVec3 as Color};

use crate::{
    models::{objects::global::HitRecord, ray::Ray},
    utils::math::{random_float, reflect, reflectance, refract},
};

use super::material::Material;

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: Ray,
        rec: &HitRecord,
        color_attenuation: &mut DVec3,
        scattered: &mut Ray,
    ) -> bool {
        *color_attenuation = Color::new(1.0, 1.0, 1.0);

        let ri: f64 = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction: DVec3 = ray_in.direction.normalize();

        let cos_tetha:f64 = ((-unit_direction).dot(rec.normal)).min(1.0);
        let sin_tetha:f64 = (1.0 - cos_tetha * cos_tetha).sqrt();

        let cannot_refract:bool = ri * sin_tetha > 1.0;
        let direction:DVec3 = if cannot_refract || reflectance(cos_tetha, ri) > random_float(){
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, ri)
        };
        
        *scattered = Ray::new(rec.point, direction);

        true
    }
}
