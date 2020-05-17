use std::ops;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Color{
    pub r: f32,
    pub g: f32, 
    pub b: f32
}
impl Color{
    pub fn new(r: f32, g: f32, b: f32)->Color{
        return Color{r:r, g:g, b:b};
    }
    pub fn black()->Color{
        return Color{r:0.0, g:0.0, b:0.0};
    }
    pub fn data(&self)->[u8; 3]{
        return [(self.r * 255.0).round() as u8, (self.g * 255.0).round() as u8, (self.b * 255.0).round() as u8];
    }
}
impl ops::Add<Color> for Color{
    type Output = Color;
    fn add(self, rhs: Color)->Color{
        return Color{
            r: self.r + rhs.r, 
            g: self.g + rhs.g, 
            b: self.b + rhs.b}
    }
}
impl ops::Div<f32> for Color{
    type Output = Color;
    fn div(self, rhs: f32)->Color{
        return Color{
            r: self.r / rhs, 
            g: self.g / rhs, 
            b: self.b / rhs}
    }
}
pub fn blend(a: &Color, b: &Color, t: &f32)->Color{
    return Color::new(
        a.r * t + b.r*(1.0-t),
        a.g * t + b.g*(1.0-t),
        a.b * t + b.b*(1.0-t)
    );
}