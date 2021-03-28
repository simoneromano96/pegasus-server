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
