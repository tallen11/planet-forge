pub mod xoroshiro_rng;

use util::xoroshiro_rng::XoroshiroRNG;
use renderer::primitive::vec3::Vec3;

/// Generate uniform sample on unit hemisphere.
/// Credit to https://corysimon.github.io/articles/uniformdistn-on-sphere/
/// for the algorithm.
#[inline]
pub fn sample_unit_hemisphere_uniform(rng: &mut XoroshiroRNG) -> Vec3 {
    let mut sample = Vec3::new(rng.next_f32(),
                               rng.next_f32(),
                               rng.next_f32());

    while sample.length() < 0.0001 {
        sample = Vec3::new(rng.next_f32(),
                           rng.next_f32(),
                           rng.next_f32());
    }

    sample.normalized()
}
