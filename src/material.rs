use crate::ray::{Ray, HitStruct};
use crate::color::Color;
use crate::random::random_unit_vector;
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
