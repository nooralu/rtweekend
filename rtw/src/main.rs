mod color_util;

use color_util::write_color;
use rand::prelude::*;
use raytracer::{
    camera::Camera,
    hittable::{hittable_list::HittableList, sphere::Sphere, Hittable},
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material},
    ray::Ray,
};
use std::{
    f64::{consts::PI, INFINITY},
    sync::Arc,
};
use vec3::{unit_vector, Color};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world: HittableList = Default::default();
    let material_ground: Arc<Box<dyn Material>> =
        Arc::new(Box::new(Lambertian::new_with((0.8, 0.8, 0.0).into())));
    let material_center: Arc<Box<dyn Material>> =
        Arc::new(Box::new(Lambertian::new_with((0.1, 0.2, 0.5).into())));
    let material_left: Arc<Box<dyn Material>> = Arc::new(Box::new(Dielectric::new_with(1.5)));
    let material_right: Arc<Box<dyn Material>> =
        Arc::new(Box::new(Metal::new_with((0.8, 0.6, 0.2).into(), 0.0)));

    world.add(Box::new(Sphere::new_with(
        (0.0, -100.5, -1.0).into(),
        100.0,
        material_ground,
    )));

    world.add(Box::new(Sphere::new_with(
        (0.0, 0.0, -1.0).into(),
        0.5,
        material_center,
    )));

    world.add(Box::new(Sphere::new_with(
        (-1.0, 0.0, -1.0).into(),
        0.5,
        material_left.clone(),
    )));

    world.add(Box::new(Sphere::new_with(
        (-1.0, 0.0, -1.0).into(),
        -0.4,
        material_left.clone(),
    )));

    world.add(Box::new(Sphere::new_with(
        (1.0, 0.0, -1.0).into(),
        0.5,
        material_right,
    )));

    // Camera
    let camera: Camera = Default::default();

    // Render

    print!("P3\n{image_width} {image_height}\n255\n");

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {j}");
        for i in 0..image_width {
            let mut pixel_color: Color = Default::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            write_color(&pixel_color, samples_per_pixel);
        }
    }
    eprintln!("Done.");
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

fn ray_color(r: &Ray, world: &impl Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
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
