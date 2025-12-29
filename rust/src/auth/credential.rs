use sha2::{Digest, Sha256};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordHashAlgorithm {
    Plain,
    Sha256Hex,
}

impl FromStr for PasswordHashAlgorithm {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let alg = match s.to_ascii_lowercase().as_str() {
            "sha256" | "sha256hex" => PasswordHashAlgorithm::Sha256Hex,
            _ => PasswordHashAlgorithm::Plain,
        };

        Ok(alg)
    }
}

pub fn hash_password(password: &str, mode: PasswordHashAlgorithm) -> String {
    // First round
    let first = match mode {
        PasswordHashAlgorithm::Plain => password.to_string(),
        PasswordHashAlgorithm::Sha256Hex => {
            let mut hasher = Sha256::new();
            hasher.update(password.as_bytes());
            let result = hasher.finalize();
            hex::encode(result)
        }
    };

    let head_len = 32usize;
    let head: String = first.chars().take(head_len).collect();

    let composite = format!("{}{}", password, head);

    match mode {
        PasswordHashAlgorithm::Plain => composite,
        PasswordHashAlgorithm::Sha256Hex => {
            let mut hasher = Sha256::new();
            hasher.update(composite.as_bytes());
            let result = hasher.finalize();
            hex::encode(result)
        }
    }
}

pub fn verify_password(
    input_password: &str,
    stored_password: &str,
    mode: PasswordHashAlgorithm,
) -> bool {
    let input_hashed = hash_password(input_password, mode);
    // constant-time compare not strictly necessary here but could be added
    input_hashed == stored_password
}
