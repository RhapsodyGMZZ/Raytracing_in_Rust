use glam::{DVec3, DVec3 as Color};

use crate::{
    models::{objects::global::HitRecord, ray::Ray},
    utils::math::{random_unit_vector, reflect},
};

use super::material::Material;

pub struct Metal {
    albedo: Color,
    fuzziness: f64,
}

impl Metal {
    pub fn new(color: Color, mut fuzziness:f64) -> Self {
        if fuzziness > 1.0 {
            fuzziness = 1.0;
        }
        Self { albedo: color, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: Ray,
        rec: &HitRecord,
        color_attenuation: &mut DVec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected: DVec3 = reflect(ray_in.direction, rec.normal);
        reflected = reflected.normalize() + (self.fuzziness * random_unit_vector());
        *scattered = Ray::new(rec.point, reflected);
        *color_attenuation = self.albedo;
        true
    }
}
