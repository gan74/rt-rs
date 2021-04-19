use std::ops::*;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}


impl From<f32> for Color {
    fn from(x: f32) -> Color {
        Color {
            r: x,
            g: x,
            b: x,
        }
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