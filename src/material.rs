use crate::ray::{Ray, HitStruct};
use crate::color::Color;
use crate::random::{random_unit_vector, random_in_unit_sphere};
use crate::math::{Vec3, reflect};
use rand::Rng;

pub struct ScatterResult{
    pub scattered_ray: Ray,
    pub attenuation: Color
}
pub trait Material<R: Rng>{
    fn scatter(&self, rng: &mut R, ray_in: &Ray, hit_record: &HitStruct<R>)->Option<ScatterResult>;
}

pub struct Lambertian{
    albedo: Color
}
impl Lambertian{
    pub fn new(albedo: Color)->Box<Lambertian>{
        return Box::new(Lambertian{albedo: albedo});
    }
}
impl<R: Rng> Material<R> for Lambertian{
    fn scatter(&self, rng: &mut R, _ray_in: &Ray, hit_record: &HitStruct<R>)->Option<ScatterResult>
    {
        let scatter_direction = hit_record.normal + random_unit_vector(rng);
        return Some(ScatterResult{
            scattered_ray: Ray::new(hit_record.point, scatter_direction),
            attenuation: self.albedo
        })
    }
}


pub struct Metal{
    albedo: Color,
    distort: f32
}
impl Metal{
    pub fn new(albedo: Color, distort: f32)->Box<Metal>{
        return Box::new(Metal{albedo: albedo, distort: if distort > 1.0{1.0}else{distort}});
    }
}
impl<R: Rng> Material<R> for Metal{
    fn scatter(&self, rng: &mut R, ray_in: &Ray, hit_record: &HitStruct<R>)->Option<ScatterResult>
    {
        let reflected = reflect(ray_in.direction, hit_record.normal) + random_in_unit_sphere(rng) * self.distort;
        if reflected.dot(&hit_record.normal) < 0.0{
            return None;
        }
        return Some(ScatterResult{
            scattered_ray: Ray::new(hit_record.point, reflected),
            attenuation: self.albedo
        })
    }
}