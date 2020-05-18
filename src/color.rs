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
    pub fn white()->Color{
        return Color{r:1.0, g:1.0, b:1.0};
    }
    pub fn gray()->Color{
        return Color{r:0.5, g:0.5, b:0.5};
    }
    pub fn green()->Color{
        return Color{r:0.0, g:0.75, b:0.0};
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

pub fn modulate(a: &Color, b: &Color)->Color{
    return Color::new(a.r * b.r, a.g*b.g, a.b*b.b);
}