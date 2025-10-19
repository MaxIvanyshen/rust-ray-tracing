use crate::hittable::{Hittable, HitRecord};

pub struct Sphere {
    pub center: crate::vec3::Point3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: crate::vec3::Point3, radius: f32) -> Sphere {
        let mut s = Sphere { center, radius };
        if radius < 0.0 {
           s.radius = 0.0;
        }
        s
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, tmin: f32, tmax: f32, r: &mut HitRecord) -> bool {
        let oc = self.center - *ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;

        if root <= tmin || tmax <= root {
            root = (h + sqrtd) / a;
            if root <= tmin || tmax <= root {
                return false;
            }
        }

        r.t = root;
        r.p = ray.at(r.t);

        let outward_normal = (r.p - self.center) / self.radius;
        r.set_face_normal(ray, &outward_normal);

        true
    }
}
