use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
mod math;
mod color;
mod random;
pub use crate::math::Vec3;
pub use crate::color::{Color, blend};
pub use crate::random::random_unit_vector;

const ASPECT_RATIO: f32 = 16.0/9.0;
const IMG_WIDTH: usize = 512;
const IMG_HEIGHT: usize = (IMG_WIDTH as f32 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 100;
const RAY_BIAS: f32 = 0.001;
const MAX_DEPTH: u32 = 30;

struct Ray{
    origin: Vec3,
    direction: Vec3
}
impl Ray{
    fn new(origin: Vec3, direction: Vec3)->Ray{
        return Ray{
            origin: origin,
            direction: direction.normalized()
        }
    }
    fn at(&self, t: f32)->Vec3{
        return self.origin + (self.direction * t);
    }
}
struct HitStruct{
    point: Vec3,
    normal: Vec3,
    t: f32,
    front_facing: bool
}
impl HitStruct{
    fn new(ray: &Ray, point: Vec3, outward_normal: Vec3, t: f32)->HitStruct{
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        return HitStruct{
            point: point,
            normal: if front_face{outward_normal}else{-outward_normal},
            t: t,
            front_facing: front_face
        }
    }
}
trait Hittable{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32)->Option<HitStruct>;
}

struct Sphere{
    center: Vec3,
    radius: f32
}
impl Sphere{
    fn new(center: Vec3, radius: f32)->Box<Sphere>{
        return Box::new(Sphere{center: center, radius: radius});
    }
}
impl Hittable for Sphere{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32)->Option<HitStruct>
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
                return Some(HitStruct::new(ray, pt, (pt - self.center)/self.radius, temp));
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min{
                let pt = ray.at(temp);
                return Some(HitStruct::new(ray, pt, (pt - self.center)/self.radius, temp));
            }
        }
        return None;
    }
}

struct HittableList{
    objects: Vec<Box<dyn Hittable>>
}
impl HittableList{
    fn new()->HittableList{
        return HittableList{objects: vec!()};
    }
    fn add(&mut self, object:Box<dyn Hittable>){
        self.objects.push(object);
    }
}
impl Hittable for HittableList{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32)->Option<HitStruct>{
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
    vertical: Vec3
}
impl Camera{
    fn new(aspect_ratio: f32, viewport_height: f32, focal_length: f32, origin: Vec3)->Camera{
        let viewport_width = aspect_ratio * viewport_height;
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        return Camera{
            origin: origin,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length),
            horizontal: horizontal,
            vertical: vertical
        }
    }
    fn get_ray(&self, u: f32, v: f32)->Ray{
        return Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin);
    }
}

fn ray_color<R: Rng>(ray: &Ray, world: &HittableList, depth: u32, rng: &mut R)->Color{
    if depth == 0{
        return Color::black();
    }
    match world.hit(ray, RAY_BIAS, std::f32::MAX) {
        Some(res)=>{
            let dir = res.normal + random_unit_vector(rng);
            return ray_color(&Ray::new(res.point, dir), world, depth - 1,rng)/2.0;
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

    let camera = Camera::new(ASPECT_RATIO, 2.0, 1.0, Vec3::new(0.0, 0.0, 0.0));

    let mut world = HittableList::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(0.0 .. 1.0);
    for j in (0..IMG_HEIGHT).rev(){
        println!("Scanlines remaining: {}", j);
        for i in 0..IMG_WIDTH{
            let mut c = Color::new(0.0, 0.0, 0.0);
            for s in 0..SAMPLES_PER_PIXEL{
                let u = (i as f32 + dist.sample(&mut rng))/(IMG_WIDTH as f32 - 1.0);
                let v = (j as f32 + dist.sample(&mut rng))/(IMG_HEIGHT as f32 - 1.0);
                let r = camera.get_ray(u, v);
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
