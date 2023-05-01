use vec3::Color;

pub fn write_color(pixel_color: &Color) {
    // Write the translated [0,255] value of each color component.
    println!(
        "{ir} {ig} {ib}",
        ir = (255.999 * pixel_color.x()) as i32,
        ig = (255.999 * pixel_color.y()) as i32,
        ib = (255.999 * pixel_color.z()) as i32
    );
}
