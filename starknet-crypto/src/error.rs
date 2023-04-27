mod sign_error {
    /// Errors when performing ECDSA [`sign`](fn.sign) operations
    #[derive(Debug)]
    pub enum SignError {
        InvalidMessageHash,
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
    /// Errors when performing ECDSA [`verify`](fn.verify) operations
    #[derive(Debug)]
    pub enum VerifyError {
        InvalidPublicKey,
        InvalidMessageHash,
        InvalidR,
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
    /// Errors when performing ECDSA [`recover`](fn.recover) operations
    #[derive(Debug)]
    pub enum RecoverError {
        InvalidMessageHash,
        InvalidR,
        InvalidS,
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
