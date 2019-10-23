pub mod vec3 {
    #[derive(Debug, Copy, Clone)]
    pub struct Vec3 {
        x: f32,
        y: f32,
        z: f32,
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
    }
}
