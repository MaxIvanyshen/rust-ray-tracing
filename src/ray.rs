mod vec3;
use vec3::{Point3, Vec3};

pub struct Ray {
    origin: &Point3,
    direction: &Vec3,
}

pub fn new(origin: &Point3, direction: &Vec3) -> Ray {
    Ray {
        origin,
        direction,
    }
}

impl Ray {
    pub fn origin(self) -> &Point3 {
        self.origin 
    }

    pub fn direction(self) -> &Vec3 {
        self.direction 
    }

    pub fn at(self, t: f32) -> vec::Point3 {
        self.origin + t * self.direction
    }
}
