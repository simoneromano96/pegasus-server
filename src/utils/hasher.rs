use blake3::{Hash, Hasher as BlakeHasher};
use djangohashers::{check_password, make_password_with_algorithm, Algorithm::Argon2};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PasswordErrors {
  #[error("Hashing error")]
  HashError,
  #[error("Invalid password")]
  InvalidPassword,
}

/// Hashes a password
///
/// https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
pub fn hash_password(password: &str) -> String {
  make_password_with_algorithm(password, Argon2)
}

/// Verify a password, gives Ok if the password is verified else the error
///
/// https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
pub fn verify_password(password: &str, encoded: &str) -> Result<(), PasswordErrors> {
  match check_password(password, encoded) {
    Ok(valid) => {
      if valid {
        Ok(())
      } else {
        Err(PasswordErrors::InvalidPassword)
      }
    }
    Err(_) => Err(PasswordErrors::HashError),
  }
}

/// Generic hash with dynamic digest length
///
/// writes to `out`, prefer an output of at least 32 (so 32-bytes or 256-bits) when possible, MUST be at least 1 otherwise will panic
pub fn hash_data(data: &[u8], out: &'_ mut [u8]) {
  // Create a BLAKE 3 hasher
  let mut hasher = BlakeHasher::new();

  // Write input message
  hasher.update(data);

  // Create reader
  let mut output_reader = hasher.finalize_xof();

  // Fill the `out` buffer
  output_reader.fill(out);
}
