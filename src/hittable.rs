use std::sync::Arc;

mod vec3;


pub struct HitRecord {
    pub p: vec3::Point3,
    pub normal: vec3::Vec3,
    
    pub front_face: bool,
}

impl HitRecord {
   fn set_face_normal(&self, r: &vec3::Ray, outward_normal: &vec3::Vec3) {
       // Sets the hit record normal vector.
       // NOTE: the parameter `outward_normal` is assumed to have unit length.

       self.front_face = r.direction().dot(outward_normal) < 0;
       self.normal = if self.front_face { outward_normal } else { -outward_normal };
   }
}

pub trait Hittable {
    fn hit(&self, ray: &ray::Ray, tmin: f32, tmax: f32, r: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Arc<Hittable>>,
}

fn new_list() -> HittableList {
    return HittableList {
        objects: vec![],  
    }
}

fn new_list(object: Arc<Hittable>) -> HittableList {
    return HittableList {
        objects: vec![object],  
    }
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            p: vec3::Point3::new(0.0, 0.0, 0.0),
            normal: vec3::Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
        };
        let mut hit_anything = false;
        let mut closest_so_far = tmax;

        for object in &self.objects {
            if object.hit(r, tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        return hit_anything;
    }
}

