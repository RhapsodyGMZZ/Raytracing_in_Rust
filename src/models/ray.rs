use glam::{DVec3,DVec3 as Point};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: DVec3,
}

impl Ray {
    pub fn new(origin: Point, direction: DVec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
}
