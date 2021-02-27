use djangohashers::check_password;
pub use djangohashers::{make_password_with_algorithm, Algorithm};

pub fn hash_password(password: &str) -> String {
    make_password_with_algorithm(password, Algorithm::Argon2)
}

pub fn verify_password(password: &str, encoded: &str) -> bool {
    let result = check_password(password, encoded);
    match result {
        Ok(r) => r,
        Err(_) => false,
    }
}
