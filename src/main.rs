mod primitives;
pub use crate::primitives::vec3::*;

fn main() {
    let _vector = Vec3::zero();
    println!("{}", _vector);
}
