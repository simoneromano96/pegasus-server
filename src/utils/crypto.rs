use chacha20poly1305::{
  aead::{Aead, NewAead},
  Key, XChaCha20Poly1305, XNonce,
};
use log::debug;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

use super::hash_data;

/// This function takes a secret key and some data and gives back the encrypted text
///
/// It works by hashing the key with SHA3 256 and feeding it to a XChaCha20Poly1305 Cipher
///
/// The cipher nonce is generated randomly via a ChaCha20 Pseudo Random Number Generator (which is cryptographically secure) with the entropy given by the OS
///
/// The input data then is added to the cipher and given back
///
/// References for the Cipher: https://tools.ietf.org/html/rfc7539#section-1
///
/// An alternative random generator is: https://rust-random.github.io/rand/rand_hc/struct.Hc128Rng.html
pub fn encrypt_data(key: &[u8], data: &[u8]) -> Vec<u8> {
  debug!("Encrypting: {:?} with {:?}", &data, &key);

  // Read hash digest
  let hashed_key = hash_data(key);

  debug!("{:?}", &hashed_key);

  // The key MUST be 32-bytes or 256-bits
  let key = Key::from_slice(hashed_key.as_bytes());

  // Create the cipher with the given key
  let cipher = XChaCha20Poly1305::new(key);

  // Generate a random nonce getting entropy from the OS
  let mut rng = ChaCha20Rng::from_entropy();
  let random_nonce: [u8; 24] = rng.gen();

  debug!("{:?}", &random_nonce);

  // The nonce MUST be 24-bytes or 192-bits and unique
  let nonce = XNonce::from_slice(&random_nonce);

  // Finally encrypt the text
  let ciphertext = cipher.encrypt(nonce, data).expect("encryption failure!");

  debug!("{:?}", &ciphertext);

  ciphertext
}

pub fn decrypt_data(key: &[u8], data: &[u8]) -> Result<Vec<u8>, chacha20poly1305::aead::Error> {
  debug!("Decrypting: {:?} with {:?}", &data, &key);

  // Read hash digest
  let hashed_key = hash_data(key);

  debug!("{:?}", &hashed_key);

  // The key MUST be 32-bytes or 256-bits
  let key = Key::from_slice(hashed_key.as_bytes());

  // Create the cipher with the given key
  let cipher = XChaCha20Poly1305::new(key);

  // The nonce MUST be 24-bytes or 192-bits and unique
  let nonce = XNonce::from_slice(b"");

  cipher.decrypt(nonce, data)
}
