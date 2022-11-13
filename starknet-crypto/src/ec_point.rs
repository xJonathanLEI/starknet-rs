use crate::{
    pedersen_params::{ALPHA, BETA},
    FieldElement,
};

/// A point on an elliptic curve over [FieldElement].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct EcPoint {
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

impl EcPoint {
    pub fn from_x(x: FieldElement) -> Self {
        let y_squared = x * x * x + ALPHA * x + BETA;
        Self {
            x,
            y: y_squared.sqrt().unwrap(), // TODO: check if calling `unwrap()` here is safe
            infinity: false,
        }
    }

    fn identity() -> EcPoint {
        Self {
            x: FieldElement::ZERO,
            y: FieldElement::ZERO,
            infinity: true,
        }
    }

    fn double(&self) -> EcPoint {
        if self.infinity {
            return *self;
        }

        // l = (3x^2+a)/2y with a=1 from stark curve
        let lambda = {
            let dividend = FieldElement::THREE * (self.x * self.x) + FieldElement::ONE;
            let divisor_inv = (FieldElement::TWO * self.y).invert().unwrap();
            dividend * divisor_inv
        };

        let result_x = (lambda * lambda) - self.x - self.x;
        let result_y = lambda * (self.x - result_x) - self.y;

        EcPoint {
            x: result_x,
            y: result_y,
            infinity: false,
        }
    }

    pub fn add(&self, other: &EcPoint) -> EcPoint {
        if other.infinity {
            return *self;
        }
        if self.infinity {
            return *other;
        }
        if self.x == other.x {
            return self.double();
        }

        // l = (y2-y1)/(x2-x1)
        let lambda = {
            let dividend = other.y - self.y;
            let divisor_inv = (other.x - self.x).invert().unwrap();
            dividend * divisor_inv
        };

        let result_x = (lambda * lambda) - self.x - other.x;
        let result_y = lambda * (self.x - result_x) - self.y;

        EcPoint {
            x: result_x,
            y: result_y,
            infinity: false,
        }
    }

    pub fn subtract(&self, other: &EcPoint) -> EcPoint {
        self.add(&EcPoint {
            x: other.x,
            y: -other.y,
            infinity: other.infinity,
        })
    }

    pub fn multiply(&self, bits: &[bool]) -> EcPoint {
        let mut product = EcPoint::identity();
        for b in bits.iter().rev() {
            product = product.double();
            if *b {
                product = product.add(self);
            }
        }

        product
    }
}

impl From<&ProjectivePoint> for EcPoint {
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
    pub const fn from_ec_point(p: &EcPoint) -> Self {
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

    pub fn double(&self) -> ProjectivePoint {
        if self.infinity {
            return *self;
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

        ProjectivePoint {
            x,
            y,
            z,
            infinity: false,
        }
    }

    pub fn add(&self, other: &ProjectivePoint) -> ProjectivePoint {
        if other.infinity {
            return *self;
        }
        if self.infinity {
            return *other;
        }
        let u0 = self.x * other.z;
        let u1 = other.x * self.z;
        if u0 == u1 {
            return self.double();
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

        ProjectivePoint {
            x,
            y,
            z,
            infinity: false,
        }
    }

    pub fn multiply(&self, bits: &[bool]) -> ProjectivePoint {
        let mut product = ProjectivePoint::identity();
        for b in bits.iter().rev() {
            product = product.double();
            if *b {
                product = product.add(self);
            }
        }

        product
    }
}

impl From<&EcPoint> for ProjectivePoint {
    fn from(p: &EcPoint) -> Self {
        Self::from_ec_point(p)
    }
}
