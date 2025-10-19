use std::io::Result;

use crate::{color::Color};

mod color;
mod vec3;
mod ray;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;

static IMAGE_HEIGHT: i32 = match (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32 {
    h if h >= 1 => h,
    _ => 1,
};

const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32);

fn hit_sphere(center: &vec3::Point3, radius: f32, r: &ray::Ray) -> f32 {
    let oc = *center - *r.origin();
    let a = r.direction().length_squared();
    let h = r.direction().dot(&oc);
    let c = oc.length_squared() - (radius * radius);
    let discriminant = h*h - a*c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - (discriminant as f64).sqrt() as f32) / a;
    }
}

fn ray_color(r: &ray::Ray) -> Color {
    let t = hit_sphere(&vec3::new_point(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * color::new(n.x()+1.0, n.y()+1.0, n.z()+1.0);
    }
    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * color::new(1.0, 1.0, 1.0) + a*color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<()> {

    // Camera
    let focal_length = 1.0;
    let camera_center = vec3::new_point(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v = vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    // Calculate the horizontal and vertical delta vectors
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f32;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f32;

    let viewport_upper_left = camera_center 
        - vec3::new(0.0, 0.0, focal_length) 
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

            let ray = ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray);
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
        let origin = vec3::new_point(1.0, 2.0, 3.0);
        let direction = vec3::new(4.0, 5.0, 6.0);
        let test_ray = ray::new(origin, direction);
        let t = 2.0;
        let point = test_ray.at(t);
        assert_eq!(point, vec3::new_point(1.0 + 2.0 * 4.0, 2.0 + 2.0 * 5.0, 3.0 + 2.0 * 6.0));
    }
}
