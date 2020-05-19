use rand::Rng;
use crate::math::Vec3;
use crate::material::Material;
use crate::ray::{Ray, HitStruct, Hittable};

pub struct Sphere<R: Rng>{
    center: Vec3,
    radius: f32,
    material: Box<dyn Material<R>>
}
impl<R: Rng> Sphere<R>{
    pub fn new(material: Box<dyn Material<R>>, center: Vec3, radius: f32)->Box<Sphere<R>>{
        return Box::new(Sphere{material: material, center: center, radius: radius});
    }
}
impl<R: Rng> Hittable<R> for Sphere<R>{
    fn hit<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32)->Option<HitStruct<'a, R>>
    {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let half_b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius*self.radius;
        let discriminant = half_b*half_b - a * c;    
        if discriminant > 0.0{
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min{
                let pt = ray.at(temp);
                return Some(HitStruct::new(&*self.material, ray, pt, (pt - self.center)/self.radius, temp));
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min{
                let pt = ray.at(temp);
                return Some(HitStruct::new( &*self.material, ray, pt, (pt - self.center)/self.radius, temp));
            }
        }
        return None;
    }
}