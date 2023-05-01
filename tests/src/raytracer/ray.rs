#[cfg(test)]
mod ray_tests {
    use raytracer::ray::*;
    use vec3::*;

    #[test]
    fn test_constructor() {
        let ray: Ray = Default::default();

        assert_eq!(ray.orig, (0.0, 0.0, 0.0).into());
        assert_eq!(ray.dir, (0.0, 0.0, 0.0).into());

        let origin: Vec3 = (1.0, 2.0, 3.0).into();
        let direction: Vec3 = (4.0, 5.0, 6.0).into();
        let ray: Ray = (origin, direction).into();
        assert_eq!(ray.orig, origin);
        assert_eq!(ray.dir, direction);
    }

    #[test]
    fn test_methods() {
        let origin: Vec3 = (1.0, 2.0, 3.0).into();
        let direction: Vec3 = (4.0, 5.0, 6.0).into();
        let ray: Ray = (origin, direction).into();

        assert_eq!(ray.origin(), origin);
        assert_eq!(ray.direction(), direction);

        let ray: Ray = (origin, direction).into();

        assert_eq!(ray.at(0.0), origin);
        assert_eq!(ray.at(1.0), (5.0, 7.0, 9.0).into());
        assert_eq!(ray.at(2.0), (9.0, 12.0, 15.0).into());
    }
}
