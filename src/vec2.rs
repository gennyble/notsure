use std::ops::Div;

#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn normalize<P: Into<Vec2>>(self, other: P) -> Self {
        let other = other.into();
        Self {
            x: (self.x * 2.0) / other.x - 1.0,
            y: ((other.y - self.y) * 2.0) / other.y - 1.0,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl From<(u32, u32)> for Vec2 {
    fn from(t: (u32, u32)) -> Self {
        Self {
            x: t.0 as f32,
            y: t.1 as f32,
        }
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from(t: (f32, f32)) -> Self {
        Self { x: t.0, y: t.1 }
    }
}
