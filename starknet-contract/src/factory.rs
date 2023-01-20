use starknet_accounts::{Account, Call, Execution};
use starknet_core::types::FieldElement;

/// The default UDC address: 0x041a78e741e5af2fec34b695679bc6891742439f7afb8484ecd7766661ad02bf.
const UDC_ADDRESS: FieldElement = FieldElement::from_mont([
    15144800532519055890,
    15685625669053253235,
    9333317513348225193,
    121672436446604875,
]);

/// Selector for entrypoint `deployContract`.
const SELECTOR_DEPLOYCONTRACT: FieldElement = FieldElement::from_mont([
    18249998464715511309,
    1265649739554438882,
    1439621915307882061,
    469988280392664069,
]);

pub struct ContractFactory<A> {
    class_hash: FieldElement,
    udc_address: FieldElement,
    account: A,
}

impl<A> ContractFactory<A> {
    pub fn new(class_hash: FieldElement, account: A) -> Self {
        Self::new_with_udc(class_hash, account, UDC_ADDRESS)
    }

    pub fn new_with_udc(class_hash: FieldElement, account: A, udc_address: FieldElement) -> Self {
        Self {
            class_hash,
            udc_address,
            account,
        }
    }
}

impl<A> ContractFactory<A>
where
    A: Account,
{
    pub fn deploy<C>(
        &self,
        constructor_calldata: C,
        salt: FieldElement,
        unique: bool,
    ) -> Execution<A>
    where
        C: AsRef<[FieldElement]>,
    {
        let constructor_calldata = constructor_calldata.as_ref();

        let mut calldata = vec![
            self.class_hash,
            salt,
            if unique {
                FieldElement::ONE
            } else {
                FieldElement::ZERO
            },
            constructor_calldata.len().into(),
        ];
        constructor_calldata
            .iter()
            .for_each(|item| calldata.push(*item));

        Execution::new(
            vec![Call {
                to: self.udc_address,
                selector: SELECTOR_DEPLOYCONTRACT,
                calldata,
            }],
            &self.account,
        )
    }
}
