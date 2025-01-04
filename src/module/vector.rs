#[allow(dead_code, unused_variables, unused_imports)]
use std::ops::{Add, Sub, Mul};

// Implementations for the Vector Addition, Subtraction and Multiplication
impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f64) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Vector {
        Vector {
            x: self * vector.x,
            y: self * vector.y,
            z: self * vector.z,
        }
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}


// Implementations for the Vector Equality
impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[derive(Debug, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {

    // Create a new vector
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    // Magnitude of the vector
    pub fn magnitude(&self) -> f64 {
        ((self.x*self.x) + (self.y*self.y) + (self.z*self.z)).sqrt()
    }

    // Normalize the vector
    pub fn normalize(&self) -> Vector {
        let magnitude = self.magnitude();
        Vector {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        }
    }

    // Dot product of two vectors
    pub fn dot(&self, other: &Vector) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    // Cross product of two vectors
    pub fn cross(&self, other: &Vector) -> Vector {
        self.clone() * other.clone()
    }

    // Add two vectors
    pub fn add(&self, other: &Vector) -> Vector {
        self.clone() + other.clone()
    }

    // Scale a vector
    pub fn scale(&self, scalar: f64) -> Vector {
        self.clone() * scalar
    }
}


