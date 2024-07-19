#[cfg(feature = "server")]
/// Create a 10-char Nano ID.
pub fn create_id() -> String {
    nid::Nanoid::<10>::new().to_string()
}
