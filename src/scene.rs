use intersectable::{Intersectable, IntersectionResult};
use ray::Ray;

pub struct Scene {
    scene_objects: Vec<Box<Intersectable>>,
}

impl Scene {
    pub fn new(scene_objects: Vec<Box<Intersectable>>) -> Scene {
        Scene {
            scene_objects: scene_objects,
        }
    }

    pub fn find_intersection_in_scene(&self, ray: &Ray) -> Option<IntersectionResult> {
        let mut nearest_intersection_result: Option<IntersectionResult> = None;
        for obj in &self.scene_objects {
            if let Some(result) = obj.did_intersect_with_ray(ray) {
                if let Some(nearest_result) = nearest_intersection_result {
                    if result.t < nearest_result.t {
                        nearest_intersection_result = Some(result);
                    }
                } else {
                    nearest_intersection_result = Some(result);
                }
            }
        }

        nearest_intersection_result
    }
}
