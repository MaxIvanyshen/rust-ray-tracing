use std::io::Result;
use std::sync::Arc;

use crate::{color::Color};

mod color;
mod vec3;
mod ray;
mod hittable;
mod sphere;

use crate::hittable::Hittable;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;

static IMAGE_HEIGHT: i32 = match (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32 {
    h if h >= 1 => h,
    _ => 1,
};

const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32);

fn ray_color(r: &crate::ray::Ray, world: &crate::hittable::HittableList) -> Color {
    let hit_record = &mut crate::hittable::HitRecord {
        t: 0.0,
        p: crate::vec3::Vec3::new(0.0, 0.0, 0.0),
        normal: crate::vec3::Vec3::new(0.0, 0.0, 0.0),
        front_face: false,
    };

    if world.hit(r, 0.0, f32::INFINITY, hit_record) {
        return 0.5 * color::new(
            hit_record.normal.x() + 1.0,
            hit_record.normal.y() + 1.0,
            hit_record.normal.z() + 1.0,
        );
    }
    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * color::new(1.0, 1.0, 1.0) + a*color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<()> {

    // World
    let mut world = crate::hittable::HittableList::new();
    world.add(Arc::new(crate::sphere::Sphere::new(
        crate::vec3::Vec3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Arc::new(crate::sphere::Sphere::new(
        crate::vec3::Vec3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera
    let focal_length = 1.0;
    let camera_center = crate::vec3::Vec3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = crate::vec3::Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v = crate::vec3::Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    // Calculate the horizontal and vertical delta vectors
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f32;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f32;

    let viewport_upper_left = camera_center
        - crate::vec3::Vec3::new(0.0, 0.0, focal_length)
        - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_u);

    //Render
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining {}", (IMAGE_HEIGHT - j));
        for i in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            let ray = crate::ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray, &world);
            color::write_color(&mut std::io::stdout(), &pixel_color)?;
        }
    }

    eprint!("\rDone                        \n");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() {
        let origin = vec3::Vec3::new(1.0, 2.0, 3.0);
        let direction = vec3::Vec3::new(4.0, 5.0, 6.0);
        let test_ray = ray::new(origin, direction);
        let t = 2.0;
        let point = test_ray.at(t);
        assert_eq!(point, vec3::Vec3::new(1.0 + 2.0 * 4.0, 2.0 + 2.0 * 5.0, 3.0 + 2.0 * 6.0));
    }
}
