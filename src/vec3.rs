use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, SubAssign, Sub};
use std::fmt::Display;

use crate::utilities::{rand_double, random_double};

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Point3<T> = Vec3<T>;

impl Vec3<f64> {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn to_vec(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    #[inline]
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn cross(&self, other: Self) -> Self {
        Self::new(self.y * other.z - self.z * other.y, 
                  self.z * other.x - self.x * other.z, 
                  self.x * other.y - self.y * other.x)
    }

    #[inline]
    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    #[inline]
    pub fn rand() -> Self {
        Vec3::new(rand_double(), rand_double(), rand_double())
    }

    #[inline]
    pub fn random(min: f64, max: f64) -> Self {
        Vec3::new(random_double(min, max), random_double(min, max), random_double(min ,max))
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    #[inline]
    pub fn reflect(&self, n: Vec3<f64>) -> Vec3<f64> {
        *self - 2.0 * self.dot(n) * n
    }

    #[inline]
    pub fn refract(&self, n: Vec3<f64>, etai_over_etat: f64) -> Vec3<f64> {
        let uv = self.unit_vector();
        let cos_theta = (-uv).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = - ( 1.0 - r_out_perp.length_squared() ).abs().sqrt() * n;
        r_out_parallel + r_out_perp
    }

}

impl Add<Vec3<f64>> for Vec3<f64> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)  
    }
}

impl Add<f64> for Vec3<f64> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f64) -> Self {
        Self::new(  self.x + rhs, self.y + rhs, self.z + rhs )
    }
}

impl Add<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;
    #[inline]
    fn add(self, rhs: Vec3<f64>) -> Vec3<f64> {
        Vec3::new(self + rhs.x, self + rhs.y, self + rhs.z)
    }
}

impl Sub<Vec3<f64>> for Vec3<f64> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)  
    }
}

impl Sub<f64> for Vec3<f64> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f64) -> Self {
        Self::new(  self.x + rhs, self.y + rhs, self.z + rhs )
    }
}

impl Sub<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;
    #[inline]
    fn sub(self, rhs: Vec3<f64>) -> Vec3<f64> {
        Vec3::new(self + rhs.x, self + rhs.y, self + rhs.z)
    }
}

impl AddAssign<Vec3<f64>> for Vec3<f64> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<f64> for Vec3<f64> {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}


impl Div<f64> for Vec3<f64> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f64) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs) 
    }
}

impl DivAssign<f64> for Vec3<f64> {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Mul<Vec3<f64>> for Vec3<f64> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Vec3<f64>) -> Self {
        Self::new(  self.x * rhs.x, self.y * rhs.y, self.z * rhs.z )
    }
}


impl Mul<f64> for Vec3<f64> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f64) -> Self {
        Self::new(  self.x * rhs, self.y * rhs, self.z * rhs )
    }
}

impl Mul<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;
    #[inline]
    fn mul(self, rhs: Vec3<f64>) -> Vec3<f64> {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl MulAssign<f64> for Vec3<f64> {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Neg for Vec3<f64> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z )
    }
}

impl SubAssign<f64> for Vec3<f64> {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl <T: Display> Display for Vec3<T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

pub fn random_in_unit_sphere() -> Vec3<f64> {
    loop {
        let p = Vec3::random(-1., 1.);
        if p.length_squared() < 1.0 {
            return p
        };
    }
}

pub fn random_unit_vector() -> Vec3<f64> {
    random_in_unit_sphere().unit_vector()
}

pub fn random_in_hemisphere(normal: Vec3<f64>) -> Vec3<f64> {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_in_unit_disk() -> Vec3<f64> {
    loop {
        let p = Vec3::new(random_double(-1.0, 1.0), random_double(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 { return p };
    }
}

#[cfg(test)]
mod test {
    use super::Vec3;

    #[test]
    fn refract() {
        let r_in = Vec3::new(1.0, -1.0, 0.0);
        let n = Vec3::new(0.0, 1.0, 0.0);
        let etai_over_etat = 1.0 / 1.0;
        let refracted = r_in.refract(n, etai_over_etat); 
        assert!( (r_in.unit_vector() - refracted).length() < 0.0001 );

        let etai_over_etat = 1.5;
        print!("{:?}", r_in.refract(n, etai_over_etat))
    }
}