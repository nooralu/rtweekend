use vec3::Color;

pub fn write_color(pixel_color: &Color, samples_per_pixel: i32) -> String {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    // Write the translated [0,255] value of each color component.
    format!(
        "{ir} {ig} {ib}\n",
        ir = (255.999 * r.clamp(0.0, 0.999)) as i32,
        ig = (255.999 * g.clamp(0.0, 0.999)) as i32,
        ib = (255.999 * b.clamp(0.0, 0.999)) as i32
    )
}
