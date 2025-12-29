use uuid::Uuid;

/// Generate a UUID v4 string
pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}
