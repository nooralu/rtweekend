use std::sync::Arc;

use hittable::{hittable_list::HittableList, sphere::Sphere};
use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material};
use vec3::Point3;

pub mod camera;
pub mod hittable;
pub mod material;
pub mod ppm;
pub mod ray;

pub fn random_scene() -> HittableList {
    let mut world: HittableList = Default::default();

    let ground_material = Lambertian::new_with((0.5, 0.5, 0.5).into());
    let ground_material: Arc<Box<dyn Material>> = Arc::new(Box::new(ground_material));
    world.add(Box::new(Sphere::new_with(
        (0.0, -1000.0, 0.0).into(),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center: Point3 = (
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            )
                .into();

            if (center - (4.0, 0.2, 0.0).into()).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = vec3::Color::random() * vec3::Color::random();
                    let sphere_material = Lambertian::new_with(albedo);
                    let sphere_material: Arc<Box<dyn Material>> =
                        Arc::new(Box::new(sphere_material));
                    world.add(Box::new(Sphere::new_with(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = vec3::Color::random();
                    let fuzz = rand::random::<f64>() * 0.5;
                    let sphere_material = Metal::new_with(albedo, fuzz);
                    let sphere_material: Arc<Box<dyn Material>> =
                        Arc::new(Box::new(sphere_material));
                    world.add(Box::new(Sphere::new_with(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Dielectric::new_with(1.5);
                    let sphere_material: Arc<Box<dyn Material>> =
                        Arc::new(Box::new(sphere_material));
                    world.add(Box::new(Sphere::new_with(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Dielectric::new_with(1.5);
    let material1: Arc<Box<dyn Material>> = Arc::new(Box::new(material1));
    world.add(Box::new(Sphere::new_with(
        (0.0, 1.0, 0.0).into(),
        1.0,
        material1,
    )));

    let material2 = Lambertian::new_with((0.4, 0.2, 0.1).into());
    let material2: Arc<Box<dyn Material>> = Arc::new(Box::new(material2));
    world.add(Box::new(Sphere::new_with(
        (-4.0, 1.0, 0.0).into(),
        1.0,
        material2,
    )));

    let material3 = Metal::new_with((0.7, 0.6, 0.5).into(), 0.0);
    let material3: Arc<Box<dyn Material>> = Arc::new(Box::new(material3));
    world.add(Box::new(Sphere::new_with(
        (4.0, 1.0, 0.0).into(),
        1.0,
        material3,
    )));

    world
}
