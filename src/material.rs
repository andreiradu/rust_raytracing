use crate::ray::{Ray, HitStruct};
use crate::color::Color;
use crate::random::{random_unit_vector, random_in_unit_sphere};
use crate::math::{Vec3, reflect, refract};
use rand::distributions::{Distribution, Uniform};
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
fn schlick(cosine: f32, ref_idx: f32) -> f32{
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0*r0;
    return r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0));
}

pub struct Transparent{
    ref_idx: f32
}
impl Transparent{
    pub fn new(ref_index: f32)->Box<Transparent>{
        return Box::new(Transparent{ref_idx: ref_index});
    }
}

impl<R: Rng> Material<R> for Transparent{
    fn scatter(&self, rng: &mut R, ray_in: &Ray, hit_record: &HitStruct<R>)->Option<ScatterResult>
    {
        let relative_ior = if hit_record.front_facing {1.0/self.ref_idx} else {self.ref_idx};
        let cos_theta = (-ray_in.direction.dot(&hit_record.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        if relative_ior * sin_theta > 1.0{
            let reflected = reflect(ray_in.direction, hit_record.normal);
            return Some(ScatterResult{
                scattered_ray: Ray::new(hit_record.point, reflected),
                attenuation: Color::white()
            })
        }
        let reflect_prob = schlick(cos_theta, relative_ior);
        if Uniform::from(0.0 .. 1.0).sample(rng) < reflect_prob{
            let reflected = reflect(ray_in.direction, hit_record.normal);
            return Some(ScatterResult{
                scattered_ray: Ray::new(hit_record.point, reflected),
                attenuation: Color::white()
            })
        }
        match refract(ray_in.direction, hit_record.normal, relative_ior){
            Some(refracted)=>
                return Some(ScatterResult{
                    scattered_ray: Ray::new(hit_record.point, refracted),
                    attenuation: Color::white()
                }),
            None => {
                let reflected = reflect(ray_in.direction, hit_record.normal);
                return Some(ScatterResult{
                    scattered_ray: Ray::new(hit_record.point, reflected),
                    attenuation: Color::white()
                })
            } 
        }
    }
}