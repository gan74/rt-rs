use std::ops::*;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}


impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
        }
    }

    pub fn to_srgb(&self) -> [u8; 3] {
        [to_srgb(self.r), to_srgb(self.g), to_srgb(self.b)]
    }
}

fn to_srgb(x: f32) -> u8 {
    let gamma = x.max(0.0).powf(1.0 / 2.2);
    (gamma * 255.0).min(255.0) as u8
}


impl From<f32> for Color {
    fn from(x: f32) -> Color {
        Color::new(x, x, x)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, o: Color) -> Color {
        Color {
            r: self.r + o.r,
            g: self.g + o.g,
            b: self.b + o.b,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, o: Color) -> Color {
        Color {
            r: self.r * o.r,
            g: self.g * o.g,
            b: self.b * o.b,
        }
    }
}

impl Div for Color {
    type Output = Color;

    fn div(self, o: Color) -> Color {
        Color {
            r: self.r / o.r,
            g: self.g / o.g,
            b: self.b / o.b,
        }
    }
}

impl Add<f32> for Color {
    type Output = Color;

    fn add(self, o: f32) -> Color {
        self.add(Color::from(o))
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, o: f32) -> Color {
        self.mul(Color::from(o))
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, o: f32) -> Color {
        self.div(Color::from(o))
    }
}