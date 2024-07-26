mod sign_error {
    /// Errors when performing ECDSA [`sign`](fn.sign) operations.
    #[derive(Debug)]
    pub enum SignError {
        /// The message hash is not in the range of `[0, 2^251)`.
        InvalidMessageHash,
        /// The random `k` value results in an invalid signature. A different `k` value should be
        /// used instead, typically by using a new seed per RFC-6979.
        InvalidK,
    }

    #[cfg(feature = "std")]
    impl std::error::Error for SignError {}

    impl core::fmt::Display for SignError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::InvalidMessageHash => write!(f, "Invalid message hash"),
                Self::InvalidK => write!(f, "Invalid k"),
            }
        }
    }
}
pub use sign_error::SignError;

mod verify_error {
    /// Errors when performing ECDSA [`verify`](fn.verify) operations.
    #[derive(Debug)]
    pub enum VerifyError {
        /// The public key is not a valid point on the STARK curve.
        InvalidPublicKey,
        /// The message hash is not in the range of `[0, 2^251)`.
        InvalidMessageHash,
        /// The `r` value is not in the range of `[0, 2^251)`.
        InvalidR,
        /// The `s` value is not in the range of `[0, 2^251)`.
        InvalidS,
    }

    #[cfg(feature = "std")]
    impl std::error::Error for VerifyError {}

    impl core::fmt::Display for VerifyError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::InvalidMessageHash => write!(f, "Invalid message hash"),
                Self::InvalidPublicKey => write!(f, "Invalid public key"),
                Self::InvalidR => write!(f, "Invalid r"),
                Self::InvalidS => write!(f, "Invalid s"),
            }
        }
    }
}
pub use verify_error::VerifyError;

mod recover_error {
    /// Errors when performing ECDSA [`recover`](fn.recover) operations.
    #[derive(Debug)]
    pub enum RecoverError {
        /// The message hash is not in the range of `[0, 2^251)`.
        InvalidMessageHash,
        /// The `r` value is not in the range of `[0, 2^251)`.
        InvalidR,
        /// The `s` value is not in the range of `[0,
        /// 0x0800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f)`.
        InvalidS,
        /// The `v` value is neither `0` nor `1`.
        InvalidV,
    }

    #[cfg(feature = "std")]
    impl std::error::Error for RecoverError {}

    impl core::fmt::Display for RecoverError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::InvalidMessageHash => write!(f, "Invalid message hash"),
                Self::InvalidR => write!(f, "Invalid r"),
                Self::InvalidS => write!(f, "Invalid s"),
                Self::InvalidV => write!(f, "Invalid v"),
            }
        }
    }
}
pub use recover_error::RecoverError;
