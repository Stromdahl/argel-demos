#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn normalized(v: Self) -> Self {
        let magnitude = v.magnitude();
        if magnitude == 0.0 {
            return v;
        }
        v / magnitude
    }

    pub fn x(self) -> f64 {
        self.x
    }

    pub fn y(self) -> f64 {
        self.y
    }

    pub fn z(self) -> f64 {
        self.z
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn magnitude(self) -> f64 {
        self.dot(self).sqrt()
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        (1.0 / other) * self
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec3;

    #[test]
    fn test_unit() {
        let a = Vec3::new(2.0, 2.0, 2.0);
        let a_unit = Vec3::normalized(a.clone());
        let a_mag = a.magnitude();

        assert!(
            a_unit.magnitude().abs() - 1.0 < 1e-6,
            "Assert that the magnitude of the unit vector is close to 1"
        );
        assert!(
            (a_unit - a / a_mag).magnitude() < 1e-6,
            "Assert that the normalized vector is equal to the original vector divided by its magnitude"); 

        let a = Vec3::new(0.0, 0.0, 0.0);
        let a_unit = Vec3::normalized(a);
        assert_eq!(
            a_unit,
            Vec3::new(0.0, 0.0, 0.0),
            "Assert that the unit vector of the zero vector is itself"
        );
    }

    #[test]
    fn test_magnitude() {
        let a = Vec3::new(3.0, 4.0, 12.0);
        assert_eq!(a.magnitude(), 13.0);
    }

    #[test]
    fn test_add() {
        let a = Vec3::new(3.0, 3.0, 3.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3::new(4.0, 5.0, 6.0), a + b);
    }

    #[test]
    fn test_sub() {
        let a = Vec3::new(3.0, 3.0, 3.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3::new(2.0, 1.0, 0.0), a - b);
    }

    #[test]
    fn test_mul() {
        let a = Vec3::new(3.0, 3.0, 3.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3::new(3.0, 6.0, 9.0), a * b);
    }

    #[test]
    fn test_mul_scalar() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3::new(3.0, 6.0, 9.0), a * 3.0);
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3::new(3.0, 6.0, 9.0), 3.0 * a);
    }

    #[test]
    fn test_div() {
        let x = 2.0;
        let y = 4.0;
        let z = 8.0;
        let expect = Vec3::new(1.0, 2.0, 4.0);
        let a = Vec3::new(x, y, z);
        assert_eq!(expect, a / 2.0);
    }
}
