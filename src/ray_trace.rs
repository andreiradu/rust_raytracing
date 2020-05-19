use rand::Rng;
use crate::ray::Ray;
use crate::color::{Color, blend, modulate};
use crate::ray::Hittable;
use crate::world::World;

pub fn ray_color<R: Rng>(ray: &Ray, world: &World<R>, depth: u32, ray_bias: f32, rng: &mut R)->Color{
    if depth == 0{
        return Color::black();
    }
    match world.hit(ray, ray_bias, std::f32::MAX) {
        Some(res)=>{
            match res.material.scatter(rng, ray, &res){
                Some(scatter_res)=>{return modulate(&scatter_res.attenuation, &ray_color(&scatter_res.scattered_ray, world, depth - 1, ray_bias, rng))}
                _=>{return Color::black()}
            }
        }
        _=>{}
    } 
    let t = 0.5 * (ray.direction.y + 1.0);
    return blend(&Color::new(0.5, 0.7, 1.0), &Color::new(1.0, 1.0, 1.0), &t);
}