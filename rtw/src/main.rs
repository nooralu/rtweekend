mod color_util;

use color_util::write_color;

fn main() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{image_width} {image_height}\n255\n");

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {j}");
        for i in 0..image_width {
            let pixel_color = (
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25_f64,
            );
            write_color(&pixel_color.into());
        }
    }
    eprintln!("Done.");
}
