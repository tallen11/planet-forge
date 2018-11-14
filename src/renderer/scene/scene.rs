use renderer::object::{IntersectionResult, Intersectable};
use renderer::ray::Ray;

pub struct Scene {
    objects: Vec<Box<Intersectable>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
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
