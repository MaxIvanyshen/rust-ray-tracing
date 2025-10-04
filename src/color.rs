use std::io::{Result, Write};

pub type Color = crate::vec3::Vec3;

pub fn new(e1: f32, e2: f32, e3: f32) -> Color {
    Color {
        e: [e1, e2, e3],
    }
}

pub fn write_color(out: &mut impl Write, pixel_color: &Color) -> Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte = (255.999 * r) as i32;
    let gbyte = (255.999 * g) as i32;
    let bbyte = (255.999 * b) as i32;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)?;
    Ok(())
}
