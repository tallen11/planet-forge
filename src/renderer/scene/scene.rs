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
        let mut nearest_intersection: Option<IntersectionResult> = None;

        // let a = self.objects.iter()
        //                     .map(|object| object.detect_intersection(ray))
        //                     .filter(|result| {
        //                         match result {
        //                             Some(r) => true,
        //                             None => false,
        //                         }})
        //                     .sort_by(|res1, res2| {
        //                         match (res1, res2) {
        //                             (Some(r1), Some(r2)) => r1.get_t() < r2.get_t(),
        //                             _ => unreachable!(),
        //                         }})
        //                     .collect();

        nearest_intersection
    }
}
