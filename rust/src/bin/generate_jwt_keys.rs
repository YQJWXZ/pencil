use anyhow::{Context, Result};
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey, LineEnding};
use rsa::rand_core::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let output_dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from("fixtures")
    };

    let private_key_path = output_dir.join("jwt_dev_private.pem");
    let public_key_path = output_dir.join("jwt_dev_public.pem");

    println!("Generating RSA 2048-bit key pair...");

    // Create output directory if not exists
    fs::create_dir_all(&output_dir)
        .with_context(|| format!("Failed to create directory: {}", output_dir.display()))?;

    // Generate RSA key pair
    let mut rng = OsRng;
    let private_key =
        RsaPrivateKey::new(&mut rng, 2048).context("Failed to generate RSA private key")?;
    let public_key = RsaPublicKey::from(&private_key);

    // Encode to PEM format
    let private_pem = private_key
        .to_pkcs1_pem(LineEnding::LF)
        .context("Failed to encode private key to PEM")?
        .to_string();

    let public_pem = public_key
        .to_pkcs1_pem(LineEnding::LF)
        .context("Failed to encode public key to PEM")?;

    // Write to files
    fs::write(&private_key_path, private_pem).with_context(|| {
        format!(
            "Failed to write private key: {}",
            private_key_path.display()
        )
    })?;

    fs::write(&public_key_path, public_pem)
        .with_context(|| format!("Failed to write public key: {}", public_key_path.display()))?;

    println!("✓ Private key saved to: {}", private_key_path.display());
    println!("✓ Public key saved to:  {}", public_key_path.display());
    println!("\nKeys generated successfully!");

    Ok(())
}
