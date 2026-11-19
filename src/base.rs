use std::{any::{type_name}, fmt, ops::{Add, Div, Mul, Sub}};

#[derive(PartialEq, Copy, Clone, Debug)] 
pub struct Vector2<T = f32> {
    pub x: T,
    pub y: T
}
impl <T> Vector2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl <T: Add<Output = T>> Add for Vector2<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self{
        Self {x: self.x + other.x, y: self.y  + other.y}
    }
}
impl <T: Sub<Output = T>> Sub for Vector2<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self{
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}
impl <T: Mul<Output = T>> Mul for Vector2<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self{
        Self {x: self.x * other.x, y: self.y * other.y}
    }
}
impl <T: Div<Output = T>> Div for Vector2<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self{
        Self {x: self.x / other.x, y: self.y / other.y}
    }
}
impl <T: fmt::Display> fmt::Display for Vector2<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Vector2<{}>({},{})", type_name::<T>(), self.x, self.y)
    }
}
pub type Vector2Int = Vector2<i32>;