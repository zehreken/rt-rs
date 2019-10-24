pub mod vec3 {
    use std::fmt;

    #[derive(Debug, Copy, Clone)]
    pub struct Vec3 {
        x: f32,
        y: f32,
        z: f32,
    }

    impl fmt::Display for Vec3 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
        }
    }

    impl Vec3 {
        pub fn zero() -> Vec3 {
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }

        pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
            Vec3 { x: x, y: y, z: z }
        }

        pub fn x(self) -> f32 {
            return self.x;
        }

        pub fn y(self) -> f32 {
            return self.y;
        }

        pub fn z(self) -> f32 {
            return self.z;
        }

        pub fn r(self) -> f32 {
            return self.x;
        }

        pub fn g(self) -> f32 {
            return self.y;
        }

        pub fn b(self) -> f32 {
            return self.z;
        }

        pub fn squared_length(self) -> f32 {
            return self.x * self.x + self.y + self.y + self.z * self.z;
        }

        pub fn length(self) -> f32 {
            return self.squared_length().sqrt();
        }

        pub fn dot(a: Vec3, b: Vec3) -> f32 {
            return a.x * b.x + a.y * b.y + a.z * b.z;
        }

        pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
            return Vec3::new(
                a.y * b.z - a.z * b.y,
                -(a.x * b.z - a.z * b.x),
                a.x * b.y - a.y * b.x,
            );
        }
    }
}

#[cfg(test)]
mod vec3_tests {
    use crate::primitives::vec3::*;
    // use super::*;

    #[test]
    fn test_dot() {
        let a: Vec3 = Vec3::new(1.0, 1.0, 1.0);
        let b: Vec3 = Vec3::new(2.0, -3.0, -0.2);
        assert_eq!(Vec3::dot(a, a), 3.0);
        assert_eq!(Vec3::dot(b, b), 13.04);
        assert_eq!(Vec3::dot(a, b), -1.2);
        assert_eq!(Vec3::dot(b, a), Vec3::dot(b, a));
    }

    #[test]
    fn test_cross() {
        let a: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let b: Vec3 = Vec3::new(-3.0, -2.0, 1.0);
        assert_eq!(Vec3::cross(a, a).x(), 0.0);
        assert_eq!(Vec3::cross(b, b).y(), 0.0);
        assert_eq!(Vec3::cross(a, b).x(), 8.0);
        assert_eq!(Vec3::cross(b, a).z(), -4.0);
    }
}
