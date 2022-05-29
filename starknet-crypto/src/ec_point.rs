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

impl EcPoint {
    pub fn from_x(x: FieldElement) -> Self {
        let y_squared = x * x * x + ALPHA * x + BETA;
        Self {
            x,
            y: y_squared.sqrt().unwrap(), // TODO: check if calling `unwrap()` here is safe
            infinity: false,
        }
    }
    
    pub fn calculate_y(x: FieldElement) -> FieldElement {
        (x * x * x + ALPHA * x + BETA).sqrt().unwrap()
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
            let two = FieldElement::ONE + FieldElement::ONE;
            let three = two + FieldElement::ONE;
            let dividend = three * (self.x * self.x) + FieldElement::ONE;
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
