#[cfg(test)]
mod tests {
    use vec3::*;

    #[test]
    fn test_constructors() {
        let vec = Vec3::new();
        assert_eq!(vec, Vec3(0.0, 0.0, 0.0));

        let vec: Vec3 = Default::default();
        assert_eq!(vec, Vec3(0.0, 0.0, 0.0));

        let vec = Vec3::new_with(1.0, 2.0, 3.0);
        assert_eq!(vec, Vec3(1.0, 2.0, 3.0));

        let vec: Vec3 = (1.0, 2.0, 3.0).into();
        assert_eq!(vec, Vec3(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_methods() {
        let vec = Vec3::new_with(1.0, 2.0, 3.0);
        assert_eq!(vec.x(), 1.0);
        assert_eq!(vec.y(), 2.0);
        assert_eq!(vec.z(), 3.0);

        assert_eq!(vec.length_squared(), 14.0);
        assert_eq!(vec.length(), 14.0_f64.sqrt());
    }

    #[test]
    fn test_neg() {
        let vec = Vec3::new_with(1.0, 2.0, 3.0);
        assert_eq!(-vec, Vec3(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_index() {
        let mut vec = Vec3::new_with(1.0, 2.0, 3.0);
        assert_eq!(vec[0], 1.0);
        assert_eq!(vec[1], 2.0);
        assert_eq!(vec[2], 3.0);

        vec[0] = 4.0;
        vec[1] = 5.0;
        vec[2] = 6.0;
        assert_eq!(vec, Vec3(4.0, 5.0, 6.0));
    }

    #[test]
    #[should_panic(expected = "Index out of bounds for Vec3")]
    fn test_outbound_index() {
        let vec = Vec3::new_with(1.0, 2.0, 3.0);
        let _ = vec[3];
    }

    #[test]
    fn test_add_assign() {
        let mut vec = Vec3::new_with(1.0, 2.0, 3.0);
        vec += Vec3::new_with(4.0, 5.0, 6.0);
        assert_eq!(vec, Vec3(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_mul_assign() {
        let mut vec = Vec3::new_with(1.0, 2.0, 3.0);
        vec *= 2.0;
        assert_eq!(vec, Vec3(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_div_assign() {
        let mut vec = Vec3::new_with(1.0, 2.0, 3.0);
        vec /= 2.0;
        assert_eq!(vec, Vec3(0.5, 1.0, 1.5));
    }
}

#[cfg(test)]
mod util_tests {
    use vec3::*;

    #[test]
    fn test_add() {
        let vec1: Vec3 = (1.0, 2.0, 3.0).into();
        let vec2: Vec3 = (4.0, 5.0, 6.0).into();
        assert_eq!(vec1 + vec2, (5.0, 7.0, 9.0).into());
    }

    #[test]
    fn test_sub() {
        let vec1: Vec3 = (1.0, 2.0, 3.0).into();
        let vec2: Vec3 = (4.0, 5.0, 6.0).into();
        assert_eq!(vec1 - vec2, (-3.0, -3.0, -3.0).into());
    }

    #[test]
    fn test_mul() {
        let vec: Vec3 = (1.0, 2.0, 3.0).into();
        assert_eq!(vec * 2.0, (2.0, 4.0, 6.0).into());
        assert_eq!(2.0 * vec, (2.0, 4.0, 6.0).into());
    }

    #[test]
    fn test_div() {
        let vec: Vec3 = (1.0, 2.0, 3.0).into();
        assert_eq!(vec / 2.0, (0.5, 1.0, 1.5).into());
    }

    #[test]
    fn test_dot() {
        let vec1: Vec3 = (1.0, 2.0, 3.0).into();
        let vec2: Vec3 = (4.0, 5.0, 6.0).into();
        assert_eq!(dot(&vec1, &vec2), 32.0);
    }

    #[test]
    fn test_cross() {
        let vec1: Vec3 = (1.0, 2.0, 3.0).into();
        let vec2: Vec3 = (4.0, 5.0, 6.0).into();
        assert_eq!(cross(&vec1, &vec2), (-3.0, 6.0, -3.0).into());
    }

    #[test]
    fn test_unit_vector() {
        let vec: Vec3 = (2.0, 0.0, 0.0).into();
        assert_eq!(unit_vector(&vec), (1.0, 0.0, 0.0).into());
    }
}
