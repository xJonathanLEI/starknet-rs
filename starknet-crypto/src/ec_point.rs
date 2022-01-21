use crate::field_element::FieldElement;

use bitvec::{order::Lsb0, slice::BitSlice};
use ff::Field;

/// A point on an elliptic curve over [FieldElement].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct EcPoint {
    pub x: FieldElement,
    pub y: FieldElement,
    pub infinity: bool,
}

impl EcPoint {
    fn identity() -> EcPoint {
        Self {
            x: FieldElement::zero(),
            y: FieldElement::zero(),
            infinity: true,
        }
    }

    fn double(&self) -> EcPoint {
        if self.infinity {
            return *self;
        }

        // l = (3x^2+a)/2y with a=1 from stark curve
        let lambda = {
            let two = FieldElement::one() + FieldElement::one();
            let three = two + FieldElement::one();
            let dividend = three * (self.x * self.x) + FieldElement::one();
            let divisor_inv = (two * self.y).invert().unwrap();
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
        if self.infinity {
            return *other;
        }
        if other.infinity {
            return *self;
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

    pub fn multiply(&self, bits: &BitSlice<Lsb0, u64>) -> EcPoint {
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
