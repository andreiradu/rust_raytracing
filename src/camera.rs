
use rand::Rng;
use crate::random::random_in_unit_disk;
use crate::math::Vec3;
use crate::ray::Ray;

pub struct Camera{
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3, 
    w: Vec3,
    lens_radius: f32
}
impl Camera{
    pub fn new(aspect_ratio: f32, vfov: f32, origin: Vec3, look_at: Vec3, up: Vec3, aperture: f32, focus_dist: f32)->Camera{
        let theta = vfov.to_radians();
        let h = (0.5 * theta).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (origin - look_at).normalized();
        let u = up.cross(&w).normalized();
        let v = w.cross(&u);

        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        return Camera{
            origin: origin,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist,
            horizontal: horizontal,
            vertical: vertical,
            u: u,
            v: v,
            w: w,
            lens_radius: aperture * 0.5
        }
    }
    pub fn get_ray<R: Rng>(&self, rng: &mut R, s: f32, t: f32)->Ray{
        let rd = random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        return Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset);
    }
}