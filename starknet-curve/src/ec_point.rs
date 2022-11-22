use starknet_ff::FieldElement;

use crate::curve_params::{ALPHA, BETA};

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
    pub fn from_x(x: FieldElement) -> Self {
        let y_squared = x * x * x + ALPHA * x + BETA;
        Self {
            x,
            y: y_squared.sqrt().unwrap(), // TODO: check if calling `unwrap()` here is safe
            infinity: false,
        }
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

    pub fn add(&self, other: &AffinePoint) -> AffinePoint {
        let mut copy = *self;
        copy.add_assign(other);
        copy
    }

    pub fn add_assign(&mut self, other: &AffinePoint) {
        if other.infinity {
            return;
        }
        if self.infinity {
            self.x = other.x;
            self.y = other.y;
            self.infinity = other.infinity;
            return;
        }
        if self.x == other.x {
            self.double_assign();
            return;
        }

        // l = (y2-y1)/(x2-x1)
        let lambda = {
            let dividend = other.y - self.y;
            let divisor_inv = (other.x - self.x).invert().unwrap();
            dividend * divisor_inv
        };

        let result_x = (lambda * lambda) - self.x - other.x;
        self.y = lambda * (self.x - result_x) - self.y;
        self.x = result_x;
    }

    pub fn subtract(&self, other: &AffinePoint) -> AffinePoint {
        let mut copy = *self;
        copy.subtract_assign(other);
        copy
    }

    pub fn subtract_assign(&mut self, other: &AffinePoint) {
        self.add_assign(&AffinePoint {
            x: other.x,
            y: -other.y,
            infinity: other.infinity,
        })
    }

    pub fn multiply(&self, bits: &[bool]) -> AffinePoint {
        let mut product = AffinePoint::identity();
        for b in bits.iter().rev() {
            product.double_assign();
            if *b {
                product.add_assign(self);
            }
        }

        product
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

impl ProjectivePoint {
    pub const fn from_affine_point(p: &AffinePoint) -> Self {
        Self {
            x: p.x,
            y: p.y,
            z: FieldElement::ONE,
            infinity: false,
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

    pub fn add_assign(&mut self, other: &ProjectivePoint) {
        if other.infinity {
            return;
        }
        if self.infinity {
            self.x = other.x;
            self.y = other.y;
            self.z = other.z;
            self.infinity = other.infinity;
            return;
        }
        let u0 = self.x * other.z;
        let u1 = other.x * self.z;
        if u0 == u1 {
            self.double_assign();
            return;
        }

        let t0 = self.y * other.z;
        let t1 = other.y * self.z;
        let t = t0 - t1;

        let u = u0 - u1;
        let u2 = u * u;

        let v = self.z * other.z;
        let w = t * t * v - u2 * (u0 + u1);
        let u3 = u * u2;

        let x = u * w;
        let y = t * (u0 * u2 - w) - t0 * u3;
        let z = u3 * v;

        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn add_affine_assign(&mut self, other: &AffinePoint) {
        if other.infinity {
            return;
        }
        if self.infinity {
            self.x = other.x;
            self.y = other.y;
            self.z = FieldElement::ONE;
            self.infinity = other.infinity;
            return;
        }
        let u0 = self.x;
        let u1 = other.x * self.z;
        let t0 = self.y;
        let t1 = other.y * self.z;
        if u0 == u1 {
            if t0 != t1 {
                self.infinity = true;
                return;
            } else {
                self.double_assign();
                return;
            }
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

    pub fn multiply(&self, bits: &[bool]) -> ProjectivePoint {
        let mut product = ProjectivePoint::identity();
        for b in bits.iter().rev() {
            product.double_assign();
            if *b {
                product.add_assign(self);
            }
        }

        product
    }
}

impl From<&AffinePoint> for ProjectivePoint {
    fn from(p: &AffinePoint) -> Self {
        Self::from_affine_point(p)
    }
}
