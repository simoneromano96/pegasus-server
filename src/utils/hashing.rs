use djangohashers::{check_password_tolerant, make_password_with_algorithm, Algorithm};

/// Hashes a password
pub fn hash_password(password: &str) -> String {
	let hashed = make_password_with_algorithm(password, Algorithm::Argon2);
	hashed
}

/// Verify a password against an hash
pub fn verify_password(password: &str, encoded: &str) -> bool {
	check_password_tolerant(password, encoded)
}
