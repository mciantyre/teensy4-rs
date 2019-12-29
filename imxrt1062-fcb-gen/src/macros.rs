//! Macros shared throughout the implementation

/// Macro to automatically provide an `AsRef<[u8]>` implementation
/// for newtypes that wrap a byte array.
macro_rules! as_ref_bytes_newtype {
    ($newtype:ty) => {
        impl AsRef<[u8]> for $newtype {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
    };
}
