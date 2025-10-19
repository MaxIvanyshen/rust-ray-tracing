use std::sync::Arc;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: crate::vec3::Point3,
    pub normal: crate::vec3::Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &crate::ray::Ray, outward_normal: &crate::vec3::Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -(*outward_normal) };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &crate::ray::Ray, tmin: f32, tmax: f32, r: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
    
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }
    
    pub fn from(objects: Vec<Arc<dyn Hittable>>) -> HittableList {
        HittableList {
            objects,
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            p: crate::vec3::Vec3::new(0.0, 0.0, 0.0),
            normal: crate::vec3::Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        };
        let mut hit_anything = false;
        let mut closest_so_far = tmax;

        for object in &self.objects {
            if object.hit(ray, tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}

