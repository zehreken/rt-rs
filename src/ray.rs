pub mod ray {
    use crate::primitives::vec3::*;
    use std::fmt;

    #[derive(Debug, Copy, Clone)]
    pub struct HitRecord {
        t: f32,
        p: Vec3,
        normal: Vec3,
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Ray {
        from: Vec3,
        to: Vec3,
    }

    impl fmt::Display for Ray {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "from: {}\nto: {}", self.from, self.to)
        }
    }

    impl Ray {
        pub fn new(from: Vec3, to: Vec3) -> Ray {
            Ray { from: from, to: to }
        }

        pub fn origin(self) -> Vec3 {
            return self.from;
        }

        pub fn direction(self) -> Vec3 {
            return self.to;
        }

        pub fn point_at(self, t: f32) -> Vec3 {
            return self.from + self.to * t;
        }
    }
}
