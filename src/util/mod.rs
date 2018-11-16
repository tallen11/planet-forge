pub mod xoroshiro_rng;

use util::xoroshiro_rng::XoroshiroRNG;
use renderer::primitive::vec3::Vec3;

/// Generate uniform sample on unit hemisphere.
/// Credit to https://corysimon.github.io/articles/uniformdistn-on-sphere/
/// for the algorithm.
#[inline]
pub fn sample_unit_hemisphere_uniform(rng: &mut XoroshiroRNG) -> Vec3 {
    let mut sample = Vec3::new(2.0 * rng.next_f32() - 1.0,
                               2.0 * rng.next_f32() - 1.0,
                               2.0 * rng.next_f32() - 1.0);

    while sample.length() < 0.0001 {
        sample = Vec3::new(2.0 * rng.next_f32() - 1.0,
                           2.0 * rng.next_f32() - 1.0,
                           2.0 * rng.next_f32() - 1.0);
    }

    if sample.y() < 0.0 {
        sample = Vec3::new(sample.x(), -sample.y(), sample.z());
    }

    sample.normalized()
}

// #[inline]
// pub fn sample_unit_hemisphere_cosine(rng: &mut XoroshiroRNG) -> Vec3 {

// }
