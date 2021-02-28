use djangohashers::{check_password, make_password_with_algorithm, Algorithm};

pub fn hash_password(password: &str) -> String {
    make_password_with_algorithm(password, Algorithm::Argon2)
}

pub fn verify_password(password: &str, encoded: &str) -> bool {
    check_password(password, encoded).unwrap_or(false)
}
