use crate::math::Vec3;
use crate::Material;
use rand::Rng;

pub struct Ray{
    pub origin: Vec3,
    pub direction: Vec3
}
impl Ray{
    pub fn new(origin: Vec3, direction: Vec3)->Ray{
        return Ray{
            origin: origin,
            direction: direction.normalized()
        }
    }
    pub fn at(&self, t: f32)->Vec3{
        return self.origin + (self.direction * t);
    }
}

pub struct HitStruct<'a, R : Rng>{
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_facing: bool,
    pub material: &'a dyn  Material<R>
}
impl<'a, R : Rng> HitStruct<'a, R>{
    pub fn new(material: &'a dyn  Material<R>, ray: &Ray, point: Vec3, outward_normal: Vec3, t: f32)->HitStruct<'a, R>{
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        return HitStruct{
            point: point,
            normal: if front_face{outward_normal}else{-outward_normal},
            t: t,
            front_facing: front_face,
            material: material
        }
    }
}
pub trait Hittable< R : Rng>{
    fn hit<'a>(&'a self, ray: & Ray, t_min: f32, t_max: f32)->Option<HitStruct<'a, R>>;
}