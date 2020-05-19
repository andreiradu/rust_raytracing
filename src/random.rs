use rand::Rng;
use rand::distributions::{Distribution, Uniform};
pub use crate::math::Vec3;


pub fn random_unit_vector<R: Rng>(rng: &mut R)->Vec3{
    let dist1 = Uniform::from(0.0 ..= 2.0 * std::f32::consts::PI);
    let dist2 = Uniform::from(-1.0 ..= 1.0);
    let a:f32 = dist1.sample(rng);
    let z:f32 = dist2.sample(rng);
    let r = (1.0 - z * z).sqrt();
    return Vec3::new(r*(a.cos()), r * (a.sin()), z);
}

pub fn random_in_unit_sphere<R: Rng>(rng: &mut R)->Vec3{
    loop{
        let dist = Uniform::from(-1.0 ..= 1.0);
        let a = dist.sample(rng);
        let b = dist.sample(rng);
        let c = dist.sample(rng);
        if (a*a + b*b + c*c) <= 1.0{
            return Vec3::new(a, b, c);
        }
    }
}
pub fn random_in_unit_disk<R: Rng>(rng: &mut R)->Vec3{
    loop{
        let dist = Uniform::from(-1.0 ..= 1.0);
        let a = dist.sample(rng);
        let b = dist.sample(rng);
        if (a*a + b*b) <= 1.0{
            return Vec3::new(a, b, 0.0);
        }
    }
}
