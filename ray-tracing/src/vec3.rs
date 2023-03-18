#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn unit(v: Self) -> Self {
        let magnitude = v.magnitude();
        if magnitude == 0.0 {
            return v;
        }
        v / magnitude
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        (1.0 / rhs) * self
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec3;

    #[test]
    fn test_unit() {
        let a = Vec3::new(2.0, 2.0, 2.0);
        let a_unit = Vec3::unit(a.clone());
        let a_mag = a.magnitude();

        assert!(
            a_unit.magnitude().abs() - 1.0 < 1e-6,
            "Assert that the magnitude of the unit vector is close to 1"
        );
        assert!(
            (a_unit - a / a_mag).magnitude() < 1e-6,
            "Assert that the normalized vector is equal to the original vector divided by its magnitude"); 

        let a = Vec3::new(0.0, 0.0, 0.0);
        let a_unit = Vec3::unit(a);
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
