mod color_util;

use color_util::write_color;
use raytracer::ray::Ray;
use vec3::{unit_vector, Color, Point3, Vec3};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin: Point3 = (0.0, 0.0, 0.0).into();
    let herizontal: Vec3 = (viewport_width, 0.0, 0.0).into();
    let vertical: Vec3 = (0.0, viewport_height, 0.0).into();
    let lower_left_corner: Point3 =
        origin - herizontal / 2.0 - vertical / 2.0 - (0.0, 0.0, focal_length).into();

    // Render

    print!("P3\n{image_width} {image_height}\n255\n");

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {j}");
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = (
                origin,
                lower_left_corner + u * herizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r.into());
            write_color(&pixel_color);
        }
    }
    eprintln!("Done.");
}

fn ray_color(r: &Ray) -> Color {
    // normalized unit vector, so that -1.0 < y < 1.0
    let unit_direction = unit_vector(&r.direction());
    // scale t to 0.0 < t < 1.0
    let t = 0.5 * (unit_direction.y() + 1.0);
    // blend white and blue depending on y value
    let white_color: Color = (1.0, 1.0, 1.0).into();
    let blue_color: Color = (0.5, 0.7, 1.0).into();
    // blended_value = (1 − t) * start_value + t * end_value
    (1.0 - t) * white_color + t * blue_color
}
