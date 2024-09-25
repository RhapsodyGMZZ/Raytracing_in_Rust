use glam::{DVec3, DVec3 as Point, DVec3 as Color};

use crate::utils::{
    color::convert_to_rgb,
    math::{degrees_to_radians, random_float, random_in_unit_disk},
};

use super::{
    objects::global::{HitRecord, Hittable, HittableList},
    ray::Ray,
};

pub struct Camera {
    pub look_from: Point,
    pub look_at: Point,
    pub vup: DVec3,
    pub fov: f64,
    pub viewport_height: f64,
    pub u: DVec3,
    pub v: DVec3,
    pub w: DVec3,
    pub background: Color,
    image_width: i32,
    samples_per_pixel: f64,
    max_depth: f64,
    pixel_samples_scale: f64,
    image_height: i32,
    center: Point,
    pixel_delta_x: DVec3,
    pixel_delta_y: DVec3,
    first_pixel: Point,
    defocus_angle: f64,
    defocus_disk_u: DVec3,
    defocus_disk_v: DVec3,
    brightness: f64,
}

impl Camera {
    pub fn new(
        fov: f64,
        image_width: i32,
        look_from: Point,
        look_at: Point,
        vup: Point,
        aspect_ratio: f64,
        samples_per_pixel: f64,
        max_depth: f64,
        defocus_angle: f64,
        focus_dist: f64,
        background: Color,
        brightness: f64,
    ) -> Self {
        let mut image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

        if image_height < 1 {
            image_height = 1;
        };

        let theta: f64 = degrees_to_radians(fov);
        let h: f64 = (theta / 2.0).tan();

        let center: Point = look_from;
        let viewport_height: f64 = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w: DVec3 = (look_from - look_at).normalize();
        let u: DVec3 = (vup.cross(w)).normalize();
        let v: DVec3 = w.cross(u);

        let viewport_x: DVec3 = viewport_width * u;
        let viewport_y: DVec3 = viewport_height * -v;
        let viewport_upper_left: Point =
            center - (focus_dist * w) - viewport_x / 2.0 - viewport_y / 2.0;

        let pixel_delta_x: DVec3 = viewport_x / image_width as f64;
        let pixel_delta_y: DVec3 = viewport_y / image_height as f64;
        let first_pixel: Point = viewport_upper_left + 0.5 * (pixel_delta_x + pixel_delta_y);
        let pixel_samples_scale: f64 = 1.0 / samples_per_pixel;

        let defocus_radius: f64 = focus_dist * (degrees_to_radians(defocus_angle / 2.0)).tan();
        let defocus_disk_u: DVec3 = u * defocus_radius;
        let defocus_disk_v: DVec3 = v * defocus_radius;

        Self {
            look_from,
            look_at,
            fov,
            image_height,
            image_width,
            viewport_height,
            center,
            pixel_delta_x,
            pixel_delta_y,
            first_pixel,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
            vup,
            u,
            v,
            w,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            background,
            brightness,
        }
    }

    pub fn render(&mut self, world: &mut HittableList) {
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining {}   ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel as i32 {
                    let ray: Ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(ray, self.max_depth, world)
                }
                convert_to_rgb(self.brightness * (pixel_color * self.pixel_samples_scale));
            }
        }
    }

    pub fn ray_color(&self, ray: Ray, max_depth: f64, world: &mut HittableList) -> Color {
        if max_depth <= 0.0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec: HitRecord = HitRecord::new();

        // Color of the background of the Scene
        if !world.hit(ray, 0.001..f64::INFINITY, &mut rec) {
            return self.background;
        }

        // if the ray intersects in an object, it displays it
        let mut scattered: Ray = Ray::new(DVec3::new(0.0, 0.0, 0.0), DVec3::new(0.0, 0.0, 0.0));
        let mut color_attenuation: Color = Color::new(0.0, 0.0, 0.0);
        let color_from_emission: Color = rec.mat.color_emitted(rec.u, rec.v, rec.point);

        if !rec
            .mat
            .scatter(ray, &rec, &mut color_attenuation, &mut scattered)
        {
            return color_from_emission;
        }
        let color_from_scatter: Color =
            color_attenuation * self.ray_color(scattered, max_depth - 1.0, world);
        return color_from_emission + color_from_scatter;
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset: DVec3 = self.sample_square();
        let pixel_sample: Point = self.first_pixel
            + ((i as f64 + offset.x) * self.pixel_delta_x)
            + ((j as f64 + offset.y) * self.pixel_delta_y);

        let ray_origin: Point = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction: DVec3 = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> DVec3 {
        DVec3::new(random_float() - 0.5, random_float() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> DVec3 {
        let p: DVec3 = random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }
}
