
use std::ops;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]

pub struct Vec3{
    pub x: f32,
    pub y: f32,
    pub z: f32
}
impl ops::Neg for Vec3{
    type Output = Vec3;
    fn neg(self)->Vec3{
        return Vec3{
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl ops::Add<Vec3> for Vec3{
    type Output = Vec3;
    fn add(self, rhs: Vec3)->Vec3{
        return Vec3{
            x: self.x + rhs.x, 
            y: self.y + rhs.y, 
            z: self.z + rhs.z}
    }
}

impl ops::Sub<Vec3> for Vec3{
    type Output = Vec3;
    fn sub(self, rhs: Vec3)->Vec3{
        return Vec3{
            x: self.x - rhs.x, 
            y: self.y - rhs.y, 
            z: self.z - rhs.z}
    }
}

impl ops::Mul<f32> for Vec3{
    type Output = Vec3;
    fn mul(self, rhs: f32)->Vec3{
        return Vec3{
            x: self.x * rhs, 
            y: self.y * rhs, 
            z: self.z * rhs}
    }
}

impl ops::Div<f32> for Vec3{
    type Output = Vec3;
    fn div(self, rhs: f32)->Vec3{
        return Vec3{
            x: self.x / rhs, 
            y: self.y / rhs, 
            z: self.z / rhs}
    }
}
impl Vec3{
    pub fn new(x: f32, y: f32, z:f32)->Vec3{
        return Vec3{x: x, y:y, z:z};
    }
    pub fn dot(&self, rhs: &Vec3)->f32{
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z;
    }
    pub fn cross(&self, rhs: &Vec3)->Vec3{
        return Vec3{
            x : self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }
    pub fn length(&self)->f32{
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }
    pub fn normalized(&self)->Vec3{
        let l = self.length();
        return (*self)/l;
    }
}
pub fn reflect(v: Vec3, n: Vec3) -> Vec3{
    return v - n * v.dot(&n) *  2.0;
}