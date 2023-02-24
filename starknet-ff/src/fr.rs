use ark_ff::fields::{Fp256, MontBackend, MontConfig};

pub type Fr = Fp256<MontBackend<FrConfig, 4>>;

#[derive(MontConfig)]
#[modulus = "3618502788666131213697322783095070105623107215331596699973092056135872020481"]
#[generator = "3"]
pub struct FrConfig;
