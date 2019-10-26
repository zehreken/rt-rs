pub mod sphere {
    use crate::primitives::vec3::*;
    use crate::ray::ray::*;
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

        pub fn hit(
            self,
            ray: Ray,
            t_min: f32,
            t_max: f32, /*, hit_record: &HitRecord*/
        ) -> bool {
            let origin_to_center: Vec3 = ray.origin() - self.center;
            let a: f32 = Vec3::dot(ray.direction(), ray.direction());
            let b: f32 = 2.0 * Vec3::dot(origin_to_center, ray.direction());
            let c: f32 = Vec3::dot(origin_to_center, origin_to_center) - self.radius * self.radius;
            let discriminant: f32 = b * b - 4.0 * a * c;

            return discriminant > 0.0;
        }
    }
}
