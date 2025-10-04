use std::io::{Result, Write};

use crate::vec3;

pub type Color = crate::vec3::Vec3;

pub fn new(e1: f32, e2: f32, e3: f32) -> Color {
    vec3::new(e1, e2, e3)
}

pub fn write_color(out: &mut impl Write, pixel_color: &Color) -> Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    writeln!(out, "{} {} {}", to_byte(r), to_byte(g), to_byte(b))?;
    Ok(())
}

fn to_byte(c: f32) -> i32 {
    (255.999 * c) as i32
}
