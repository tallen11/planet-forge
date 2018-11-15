use std::collections::HashMap;

use renderer::object::{IntersectionResult, Intersectable};
use renderer::object::light::Light;
use renderer::ray::Ray;
use renderer::material::material::Material;

pub struct Scene {
    object_id_counter: u32,
    objects: Vec<Box<Intersectable>>,
}

impl<'a> Scene {
    pub fn new() -> Scene {
        Scene {
            object_id_counter: 0,
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Box<Intersectable>) {
        self.objects.push(object);
    }

    pub fn find_intersection(&self, ray: Ray) -> Option<IntersectionResult> {
        self.objects.iter()
                    .map(|obj| obj.detect_intersection(ray))
                    .fold(None, |accum, res| {
                        if let (Some(a), Some(r)) = (accum, res) {
                            if r.get_t() < a.get_t() { Some(r) } else { Some(a) }
                        } else if let Some(r) = res {
                            Some(r)
                        } else if let Some(a) = accum {
                            Some(a)
                        } else {
                            None
                        }
                    })

    }
}
