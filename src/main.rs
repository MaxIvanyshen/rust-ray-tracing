use std::io::Result;

mod color;
mod vec3;

fn main() -> Result<()> {

    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining {}", (image_height - j));
        for i in 0..image_width {
            let pixel_color = color::new(i as f32 / (image_width - 1) as f32, j as f32 / (image_height - 1) as f32, 0 as f32);
            color::write_color(&mut std::io::stdout(), &pixel_color)?;
        }
    }

    eprint!("\rDone                        \n");

    Ok(())
}
