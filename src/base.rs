use std::{any::type_name, fmt, ops::{Add, Div, Mul, Sub}};

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

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self {r,g,b}
    }
    pub fn from_hex(hex: &str) -> Self {
        Self::new(u8::from_str_radix(&hex[0..2], 16).unwrap(),u8::from_str_radix(&hex[2..4], 16).unwrap(),u8::from_str_radix(&hex[4..6], 16).unwrap())
    }
    pub fn to_hex(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
    pub fn to_tuple(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
    pub const WHITE: Self = Self::new(255, 255, 255);
    pub const BLACK: Self = Self::new(0, 0, 0);
    pub const RED: Self = Self::new(255, 0, 0);
    pub const GREEN: Self = Self::new(0, 255, 0);
    pub const BLUE: Self = Self::new(0, 0, 255);
}
impl fmt::Display for Color {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Color({},{},{})", self.r, self.g, self.b)
    }
}