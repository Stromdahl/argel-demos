use crate::Vec3;

#[derive(Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn at(self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
}
