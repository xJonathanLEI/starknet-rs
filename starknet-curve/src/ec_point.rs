use crate::curve_params::{ALPHA, BETA};
use core::ops;
use starknet_ff::FieldElement;

/// A point on an elliptic curve over [FieldElement].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct AffinePoint {
    pub x: FieldElement,
    pub y: FieldElement,
    pub infinity: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ProjectivePoint {
    pub x: FieldElement,
    pub y: FieldElement,
    pub z: FieldElement,
    pub infinity: bool,
}

impl AffinePoint {
    pub fn from_x(x: FieldElement) -> Option<Self> {
        let y_squared = x * x * x + ALPHA * x + BETA;
        y_squared.sqrt().map(|y| Self {
            x,
            y,
            infinity: false,
        })
    }

    fn identity() -> AffinePoint {
        Self {
            x: FieldElement::ZERO,
            y: FieldElement::ZERO,
            infinity: true,
        }
    }

    pub fn double_assign(&mut self) {
        if self.infinity {
            return;
        }

        // l = (3x^2+a)/2y with a=1 from stark curve
        let lambda = {
            let dividend = FieldElement::THREE * (self.x * self.x) + FieldElement::ONE;
            let divisor_inv = (FieldElement::TWO * self.y).invert().unwrap();
            dividend * divisor_inv
        };

        let result_x = (lambda * lambda) - self.x - self.x;
        self.y = lambda * (self.x - result_x) - self.y;
        self.x = result_x;
    }
}

impl From<&ProjectivePoint> for AffinePoint {
    fn from(p: &ProjectivePoint) -> Self {
        let zinv = p.z.invert().unwrap();
        Self {
            x: p.x * zinv,
            y: p.y * zinv,
            infinity: false,
        }
    }
}

impl ops::Add<&AffinePoint> for &AffinePoint {
    type Output = AffinePoint;

    fn add(self, rhs: &AffinePoint) -> Self::Output {
        let mut copy = *self;
        copy += rhs;
        copy
    }
}

impl ops::AddAssign<&AffinePoint> for AffinePoint {
    fn add_assign(&mut self, rhs: &AffinePoint) {
        if rhs.infinity {
            return;
        }
        if self.infinity {
            *self = *rhs;
            return;
        }
        if self.x == rhs.x {
            if self.y == rhs.y {
                self.double_assign();
            } else {
                *self = AffinePoint::identity();
            }
            return;
        }

        let lambda = (rhs.y - self.y) * (rhs.x - self.x).invert().unwrap();

        let result_x = lambda * lambda - self.x - rhs.x;

        self.y = lambda * (self.x - result_x) - self.y;
        self.x = result_x;
    }
}

impl ops::Neg for &AffinePoint {
    type Output = AffinePoint;

    fn neg(self) -> AffinePoint {
        AffinePoint {
            x: self.x,
            y: -self.y,
            infinity: self.infinity,
        }
    }
}

impl ops::Sub<&AffinePoint> for &AffinePoint {
    type Output = AffinePoint;

    fn sub(self, rhs: &AffinePoint) -> Self::Output {
        let mut copy = *self;
        copy -= rhs;
        copy
    }
}

impl ops::SubAssign<&AffinePoint> for AffinePoint {
    fn sub_assign(&mut self, rhs: &AffinePoint) {
        *self += &-rhs;
    }
}

impl ops::Mul<&[bool]> for &AffinePoint {
    type Output = AffinePoint;

    fn mul(self, rhs: &[bool]) -> Self::Output {
        rhs.iter()
            .rev()
            .fold(AffinePoint::identity(), |mut acc, &bit| {
                acc.double_assign();
                if bit {
                    acc += self;
                }
                acc
            })
    }
}

impl ProjectivePoint {
    pub const fn from_affine_point(p: &AffinePoint) -> Self {
        Self {
            x: p.x,
            y: p.y,
            z: FieldElement::ONE,
            infinity: p.infinity,
        }
    }

    fn identity() -> ProjectivePoint {
        Self {
            x: FieldElement::ZERO,
            y: FieldElement::ZERO,
            z: FieldElement::ONE,
            infinity: true,
        }
    }

    pub fn double_assign(&mut self) {
        if self.infinity {
            return;
        }

        // t=3x^2+az^2 with a=1 from stark curve
        let t = FieldElement::THREE * self.x * self.x + self.z * self.z;
        let u = FieldElement::TWO * self.y * self.z;
        let v = FieldElement::TWO * u * self.x * self.y;
        let w = t * t - FieldElement::TWO * v;

        let uy = u * self.y;

        let x = u * w;
        let y = t * (v - w) - FieldElement::TWO * uy * uy;
        let z = u * u * u;

        self.x = x;
        self.y = y;
        self.z = z;
    }
}

impl From<&AffinePoint> for ProjectivePoint {
    fn from(p: &AffinePoint) -> Self {
        Self::from_affine_point(p)
    }
}

impl ops::AddAssign<&AffinePoint> for ProjectivePoint {
    fn add_assign(&mut self, rhs: &AffinePoint) {
        if rhs.infinity {
            return;
        }
        if self.infinity {
            *self = Self::from_affine_point(rhs);
            return;
        }
        let u0 = self.x;
        let u1 = rhs.x * self.z;
        let t0 = self.y;
        let t1 = rhs.y * self.z;
        if u0 == u1 {
            if t0 != t1 {
                self.infinity = true;
            } else {
                self.double_assign();
            }
            return;
        }

        let t = t0 - t1;
        let u = u0 - u1;
        let u2 = u * u;

        let v = self.z;
        let w = t * t * v - u2 * (u0 + u1);
        let u3 = u * u2;

        let x = u * w;
        let y = t * (u0 * u2 - w) - t0 * u3;
        let z = u3 * v;

        self.x = x;
        self.y = y;
        self.z = z;
    }
}

impl ops::AddAssign<&ProjectivePoint> for ProjectivePoint {
    fn add_assign(&mut self, rhs: &ProjectivePoint) {
        if rhs.infinity {
            return;
        }
        if self.infinity {
            *self = *rhs;
            return;
        }
        let u0 = self.x * rhs.z;
        let u1 = rhs.x * self.z;
        if u0 == u1 {
            if self.y == rhs.y {
                self.double_assign();
            } else {
                *self = ProjectivePoint::identity();
            }
            return;
        }

        let t0 = self.y * rhs.z;
        let t1 = rhs.y * self.z;
        let t = t0 - t1;

        let u = u0 - u1;
        let u2 = u * u;

        let v = self.z * rhs.z;
        let w = t * t * v - u2 * (u0 + u1);
        let u3 = u * u2;

        let x = u * w;
        let y = t * (u0 * u2 - w) - t0 * u3;
        let z = u3 * v;

        self.x = x;
        self.y = y;
        self.z = z;
    }
}

impl ops::Mul<&[bool]> for &ProjectivePoint {
    type Output = ProjectivePoint;

    fn mul(self, rhs: &[bool]) -> Self::Output {
        rhs.iter()
            .rev()
            .fold(ProjectivePoint::identity(), |mut acc, &bit| {
                acc.double_assign();
                if bit {
                    acc += self;
                }
                acc
            })
    }
}
