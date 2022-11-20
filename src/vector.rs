use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn from_array(arr: [f32; 3]) -> Vector3 {
        Vector3::new(arr[0], arr[1], arr[2])
    }

    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn default() -> Vector3 {
        Vector3::new(0.0, 0.0, 1.0)
    }

    pub fn norm(&self) -> Vector3 {
        let len = self.len();
        Vector3::new(self.x / len, self.y / len, self.z / len)
    }

    pub fn multiply(&self, coeff: f32) -> Vector3 {
        Vector3::new(self.x * coeff, self.y * coeff, self.z * coeff)
    }

    pub fn cos(a: Vector3, b: Vector3) -> f32 {
        (a * b) / (a.len() * b.len())
    }

    pub fn cross(&self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vector3 {
    type Output = f32;
    fn mul(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z + other.z
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Vector3 {}
