pub mod sphere {
    use crate::primitives::vec3::*;
    use crate::ray::ray::*;
    use std::fmt;
    use std::vec::Vec;

    pub trait Hitable {
        fn hit(self, ray: Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
    }

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

    impl Hitable for Sphere {
        fn hit(self, ray: Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
            let origin_to_center: Vec3 = ray.origin() - self.center;
            let a: f32 = Vec3::dot(ray.direction(), ray.direction());
            let b: f32 = Vec3::dot(origin_to_center, ray.direction());
            let c: f32 = Vec3::dot(origin_to_center, origin_to_center) - self.radius * self.radius;
            let discriminant: f32 = b * b - a * c;

            if discriminant > 0.0 {
                let mut temp: f32 = (-b - discriminant.sqrt()) / a;
                if temp > t_min && temp < t_max {
                    hit_record.t = temp;
                    hit_record.p = ray.point_at(temp);
                    hit_record.normal = (hit_record.p - self.center) / self.radius;

                    return true;
                }
                temp = (-b + discriminant.sqrt()) / a;
                if temp > t_min && temp < t_max {
                    hit_record.t = temp;
                    hit_record.p = ray.point_at(temp);
                    hit_record.normal = (hit_record.p - self.center) / self.radius;

                    return true;
                }
            }

            return false;
        }
    }

    impl Sphere {
        pub fn new(center: Vec3, radius: f32) -> Sphere {
            Sphere {
                center: center,
                radius: radius,
            }
        }
    }

    pub struct HitableList {
        pub list: Vec<Sphere>,
    }

    impl HitableList {
        pub fn new() -> HitableList {
            HitableList { list: Vec::new() }
        }
    }

    impl Hitable for HitableList {
        fn hit(self, ray: Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
            let mut temp_record: HitRecord = HitRecord::new();
            let mut has_hit = false;
            let mut closest_so_far: f32 = t_max;
            for obj in self.list.iter() {
                if obj.hit(ray, t_min, closest_so_far, &mut temp_record) {
                    has_hit = true;
                    closest_so_far = temp_record.t;
                    *hit_record = temp_record;
                }
            }

            return has_hit;
        }
    }
}
