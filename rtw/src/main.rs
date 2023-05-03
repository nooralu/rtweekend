use rand::prelude::*;
use rayon::prelude::*;
use raytracer::{camera::Camera, hittable::Hittable, ppm::PPMGenerator, random_scene, ray::Ray};
use std::{f64::INFINITY, thread::spawn};
use vec3::{unit_vector, Color, Point3};

fn main() {
    let aspect_ratio = 3.0 / 2.0;

    // World
    let world = random_scene();

    // Camera
    let look_from: Point3 = (13.0, 2.0, 3.0).into();
    let look_at: Point3 = (0.0, 0.0, 0.0).into();
    let vup: Point3 = (0.0, 1.0, 0.0).into();
    let dist_to_focus: f64 = 10.0;
    let aperture: f64 = 0.1;

    let camera: Camera = Camera::new_with(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render
    let mut ppm_generator = PPMGenerator::new_with(1200, aspect_ratio, 500, 50);
    let height = ppm_generator.height();
    let width = ppm_generator.width();
    let max_depth = ppm_generator.max_depth();
    let samples_per_pixel = ppm_generator.samples_per_pixel();
    let sender = ppm_generator.sender();

    let handler = spawn(move || {
        ppm_generator.save();
    });

    // Parallelize the rendering
    (0..height)
        .rev()
        .par_bridge()
        .for_each_with(sender, |sender, j| {
            let sender = sender.clone();
            (0..width).par_bridge().for_each_with(sender, |sender, i| {
                let mut pixel_color: Color = Default::default();
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + random::<f64>()) / (width - 1) as f64;
                    let v = (j as f64 + random::<f64>()) / (height - 1) as f64;
                    let r = camera.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, max_depth);
                }
                sender.send(((i, j), pixel_color)).unwrap();
            });
        });

    handler.join().unwrap();
}

fn ray_color(r: &Ray, world: &impl Hittable, depth: u32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        // black
        return (0.0, 0.0, 0.0).into();
    }

    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        let mut scattered: Ray = Default::default();
        let mut attenuation: Color = Default::default();
        if rec
            .material
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            // black
            return (0.0, 0.0, 0.0).into();
        }
    }

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
