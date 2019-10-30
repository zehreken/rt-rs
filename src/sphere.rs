pub mod sphere {
    use crate::primitives::vec3::*;
    use crate::ray::ray::*;
    use crate::utility::utility::*;
    use std::fmt;

    pub trait Hitable {
        fn hit(self, ray: Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Sphere {
        center: Vec3,
        radius: f32,
        material: u8,
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
        pub fn new(center: Vec3, radius: f32, material: u8) -> Sphere {
            Sphere {
                center: center,
                radius: radius,
                material: material,
            }
        }

        pub fn scatter(self) -> bool {
            if self.material == 0 {
                return self.lambertian();
            } else if self.material == 1 {
                return self.metal();
            } else {
                return self.lambertian(); // default is lambertian
            }
        }

        fn lambertian(self// ray: Ray,
            // hit_record: HitRecord,
            // attenuation: Vec3,
            // scattered: Ray,
        ) -> bool {
            // let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
            return true;
        }

        fn metal(self) -> bool {
            return true;
        }
    }
}
