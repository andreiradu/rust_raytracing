use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
mod math;
mod color;
mod random;
mod ray;
mod material;
use crate::random::random_in_unit_disk;
use crate::math::Vec3;
use crate::color::{Color, blend, modulate};
use crate::ray::{Ray, HitStruct, Hittable};
use crate::material::{Material,Lambertian, Metal, Transparent};

const ASPECT_RATIO: f32 = 16.0/9.0;
const IMG_WIDTH: usize = 512;
const IMG_HEIGHT: usize = (IMG_WIDTH as f32 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 100;
const RAY_BIAS: f32 = 0.001;
const MAX_DEPTH: u32 = 50;


struct Sphere<R: Rng>{
    center: Vec3,
    radius: f32,
    material: Box<dyn Material<R>>
}
impl<R: Rng> Sphere<R>{
    fn new(material: Box<dyn Material<R>>, center: Vec3, radius: f32)->Box<Sphere<R>>{
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

struct HittableList<R: Rng>{
    objects: Vec<Box<dyn Hittable<R>>>
}
impl<R: Rng> HittableList<R>{
    fn new()->HittableList<R>{
        return HittableList{objects: vec!()};
    }
    fn add(&mut self, object:Box<dyn Hittable<R>>){
        self.objects.push(object);
    }
}
impl<R: Rng> Hittable<R> for HittableList<R>{
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
struct Camera{
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
    fn new(aspect_ratio: f32, vfov: f32, origin: Vec3, look_at: Vec3, up: Vec3, aperture: f32, focus_dist: f32)->Camera{
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
    fn get_ray<R: Rng>(&self, rng: &mut R, s: f32, t: f32)->Ray{
        let rd = random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        return Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset);
    }
}

fn ray_color<R: Rng>(ray: &Ray, world: &HittableList<R>, depth: u32, rng: &mut R)->Color{
    if depth == 0{
        return Color::black();
    }
    match world.hit(ray, RAY_BIAS, std::f32::MAX) {
        Some(res)=>{
            match res.material.scatter(rng, ray, &res){
                Some(scatter_res)=>{return modulate(&scatter_res.attenuation, &ray_color(&scatter_res.scattered_ray, world, depth - 1, rng))}
                _=>{return Color::black()}
            }
        }
        _=>{}
    } 
    let t = 0.5 * (ray.direction.y + 1.0);
    return blend(&Color::new(0.5, 0.7, 1.0), &Color::new(1.0, 1.0, 1.0), &t);
}

fn main() {
    let path = Path::new("test.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    
    let mut encoder = png::Encoder::new(w, IMG_WIDTH as u32, IMG_HEIGHT as u32); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let mut data = Vec::with_capacity(IMG_WIDTH * IMG_HEIGHT * 4);

    let origin = Vec3::new(3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(ASPECT_RATIO, 25.0, origin, look_at, up, 1.0, (look_at - origin).length());

    let mut world = HittableList::new();

    world.add(Sphere::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)), Vec3::new(0.0, -100.5, -1.0), 100.0));
    world.add(Sphere::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)), Vec3::new(0.0, 0.0, -1.0), 0.5));

    world.add(Sphere::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0), Vec3::new(1.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Transparent::new(1.5), Vec3::new(-1.0, 0.0, -1.0), -0.45));
    world.add(Sphere::new(Transparent::new(1.5), Vec3::new(-1.0, 0.0, -1.0), 0.5));
    //world.add(Sphere::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3), Vec3::new(-1.0, 0.0, -1.0), 0.5));

    let mut rng = rand::thread_rng();
    let dist = Uniform::from(0.0 .. 1.0);
    for j in (0..IMG_HEIGHT).rev(){
        println!("Scanlines remaining: {}", j);
        for i in 0..IMG_WIDTH{
            let mut c = Color::new(0.0, 0.0, 0.0);
            for s in 0..SAMPLES_PER_PIXEL{
                let u = (i as f32 + dist.sample(&mut rng))/(IMG_WIDTH as f32 - 1.0);
                let v = (j as f32 + dist.sample(&mut rng))/(IMG_HEIGHT as f32 - 1.0);
                let r = camera.get_ray(&mut rng, u, v);
                c = c + ray_color(&r, &world, MAX_DEPTH, &mut rng);
            }
            c = c / (SAMPLES_PER_PIXEL as f32);
            c.r = c.r.sqrt();
            c.g = c.g.sqrt();
            c.b = c.b.sqrt();
            data.extend(&c.data());
        }
    }
    println!("Done");
    writer.write_image_data(&data).unwrap(); // Save
}
