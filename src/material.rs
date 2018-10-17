

pub trait Material {
    fn brdf(&self) -> f64;
}
