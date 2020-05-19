use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use rand::distributions::{Distribution, Uniform};
mod math;
mod color;
mod random;
mod ray;
mod material;
mod camera;
mod primitives;
mod world;
mod ray_trace;

use crate::world::World;
use crate::primitives::Sphere;
use crate::math::Vec3;
use crate::camera::Camera;
use crate::color::{Color, modulate};
use crate::material::{Material,Lambertian, Metal, Transparent};

const ASPECT_RATIO: f32 = 16.0/9.0;
const IMG_WIDTH: usize = 512;
const IMG_HEIGHT: usize = (IMG_WIDTH as f32 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 10;
const RAY_BIAS: f32 = 0.001;
const MAX_DEPTH: u32 = 50;

fn main() {
    let path = Path::new("test.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    
    let mut encoder = png::Encoder::new(w, IMG_WIDTH as u32, IMG_HEIGHT as u32); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let mut data = Vec::with_capacity(IMG_WIDTH * IMG_HEIGHT * 4);

    let origin = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(ASPECT_RATIO, 25.0, origin, look_at, up, 0.1, 10.0);

    let mut world = World::new();

    world.add(Sphere::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)), Vec3::new(0.0, -1000.0, 0.0), 1000.0));
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(0.0 .. 1.0);
    let mat_dist = Uniform::from(0..20);
    for a in -11..11{
        for b in -11..11{
            let mat = mat_dist.sample(&mut rng);
            let center = Vec3::new(a as f32 + 0.9 * dist.sample(&mut rng), 0.2, b as f32 + 0.9 * dist.sample(&mut rng));
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9{
                match mat{
                    0..=15 => {
                        let albedo = modulate(&Color::random(&mut rng),  &Color::random(&mut rng));
                        world.add(Sphere::new(Lambertian::new(albedo), center, 0.2));
                    },
                    16..=18 => {
                        let albedo = Color::random_metal(&mut rng);
                        let fuzz = Uniform::from(0.0..0.5).sample(&mut rng);
                        world.add(Sphere::new( Metal::new(albedo, fuzz), center, 0.2));
                    }
                    _ =>{
                        world.add(Sphere::new(Transparent::new(1.5), center, 0.2));
                    }
                };
                
            }
        }
    }
    world.add(Sphere::new(Transparent::new(1.5), Vec3::new(0.0, 1.0, 0.0), 1.0));
    world.add(Sphere::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)), Vec3::new(-4.0, 1.0, 0.0), 1.0));
    world.add(Sphere::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0), Vec3::new(4.0, 1.0, 0.0), 1.0));

    
    
    for j in (0..IMG_HEIGHT).rev(){
        println!("Scanlines remaining: {}", j);
        for i in 0..IMG_WIDTH{
            let mut c = Color::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL{
                let u = (i as f32 + dist.sample(&mut rng))/(IMG_WIDTH as f32 - 1.0);
                let v = (j as f32 + dist.sample(&mut rng))/(IMG_HEIGHT as f32 - 1.0);
                let r = camera.get_ray(&mut rng, u, v);
                c = c + ray_trace::ray_color(&r, &world, MAX_DEPTH, RAY_BIAS, &mut rng);
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
