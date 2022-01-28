pub mod base64 {
    use serde::Serializer;

    pub fn serialize<S, T>(value: T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: AsRef<Vec<u8>>,
    {
        serializer.serialize_str(&base64::encode(value.as_ref()))
    }
}
