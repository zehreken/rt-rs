pub mod sphere {
    use crate::primitives::vec3::*;
    use std::fmt;

    #[derive(Debug, Copy, Clone)]
    pub struct Sphere {
        center: Vec3,
        radius: f32,
    }

    impl fmt::Display for Sphere {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "sphere")
        }
    }

    impl Sphere {
        pub fn new(center: Vec3, radius: f32) -> Sphere {
            Sphere {
                center: center,
                radius: radius,
            }
        }

        pub fn hit() -> bool {
            return false;
        }
    }
}
