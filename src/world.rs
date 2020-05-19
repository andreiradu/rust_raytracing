use rand::Rng;
use crate::ray::{Ray, HitStruct, Hittable};

pub struct World<R: Rng>{
    objects: Vec<Box<dyn Hittable<R>>>
}
impl<R: Rng> World<R>{
    pub fn new()->World<R>{
        return World{objects: vec!()};
    }
    pub fn add(&mut self, object:Box<dyn Hittable<R>>){
        self.objects.push(object);
    }
}
impl<R: Rng> Hittable<R> for World<R>{
    fn hit<'a>(&'a self, ray: & Ray, t_min: f32, t_max: f32)->Option<HitStruct<'a, R>>{
        let mut ret = None;
        let mut closeset_so_far = t_max;
        for obj in &self.objects{
            match obj.hit(ray, t_min, closeset_so_far){
                Some(hit)=>{
                    closeset_so_far = hit.t;
                    ret = Some(hit);
                }
                _=>{}
            }
        }
        return ret;
    }
}