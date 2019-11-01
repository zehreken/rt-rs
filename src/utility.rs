pub mod utility {
    use crate::primitives::vec3::*;
    use rand::Rng;

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut rng = rand::thread_rng();
        let mut p: Vec3 = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            - Vec3::new(1.0, 1.0, 1.0);
        while Vec3::dot(p, p) >= 1.0 {
            p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
                - Vec3::new(1.0, 1.0, 1.0);
        }

        return p;
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        // n = unit vector
        return v - 2.0 * Vec3::dot(v, n) * n;
    }
}
