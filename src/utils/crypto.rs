use chacha20poly1305::{
  aead::{Aead, NewAead},
  Key, XChaCha20Poly1305, XNonce,
};
use log::debug;

use super::hash_data;

/// This function takes a secret key and some data and gives back the encrypted text
///
/// It works by hashing the key with SHA3 256 and feeding it to a XChaCha20Poly1305 Cipher
///
/// The cipher nonce depends on the user, each edit increments the nonce
///
/// The input data then is added to the cipher and given back
///
/// References for the Cipher: https://tools.ietf.org/html/rfc7539#section-1
pub fn encrypt_data(key: &[u8], data: &[u8], nonce: u64) -> Vec<u8> {
  debug!("Encrypting: {:?} with {:?}", &data, &key);

  // Initialize an empty array
  let mut hashed_key: [u8; 32] = [0; 32];

  // Hash
  hash_data(key, &mut hashed_key);

  debug!("{:?}", &hashed_key);

  // The key MUST be 32-bytes or 256-bits
  let key = Key::from_slice(&hashed_key);

  // Create the cipher with the given key
  let cipher = XChaCha20Poly1305::new(key);

  // Initialize an empty array
  let mut hashed_nonce: [u8; 24] = [0; 24];
  
  // Hash
  hash_data(&nonce.to_be_bytes(), &mut hashed_nonce);

  debug!("{:?}", &hashed_nonce);

  // The nonce MUST be 24-bytes or 192-bits and unique
  let nonce = XNonce::from_slice(&hashed_nonce);

  // Finally encrypt the text
  let ciphertext = cipher.encrypt(nonce, data).expect("encryption failure!");

  debug!("{:?}", &ciphertext);

  ciphertext
}

/*
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
*/