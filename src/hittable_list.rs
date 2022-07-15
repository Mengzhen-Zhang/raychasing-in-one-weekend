use std::sync::Arc;

use crate::hittable::*;

pub struct HittableList<'a>
{
    pub objects: Vec<Arc<dyn 'a + Hittable>>
}

impl<'a> HittableList<'a>
{
    pub fn new() -> HittableList<'a> {
        HittableList { objects: Vec::new() }
    }

    pub fn add<T>(&mut self, object: T)
    where
        T:  'static + Hittable
    {
        self.objects.push(Arc::new(object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, r: crate::ray::Ray<f64>, t_min: f64, t_max: f64, rec: &mut HitRecord<f64>) -> bool {
        let mut temp_rec = HitRecord::zero();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.as_ref().hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            };
        };

        hit_anything
    }
}