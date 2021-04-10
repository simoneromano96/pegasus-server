use chacha20poly1305::{
  aead::{Aead, NewAead},
  Key, XChaCha20Poly1305, XNonce,
};
use log::debug;
use thiserror::Error;
use wither::bson::{spec::BinarySubtype::Encrypted, Binary};

use super::hash_data;

#[derive(Debug, Error)]
pub enum CryptoErrors {
  #[error("Decryption error")]
  DecryptionError,
}

/// This function takes a secret key and some data and gives back the encrypted text
///
/// It works by hashing the key with SHA3 256 and feeding it to a XChaCha20Poly1305 Cipher
///
/// The cipher nonce depends on the user and is randomically generated when the user is created
///
/// The input data then is added to the cipher and given back
///
/// References for the Cipher: https://tools.ietf.org/html/rfc7539#section-1
pub fn encrypt_data(key: &[u8], nonce: &[u8], data: &[u8]) -> Vec<u8> {
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
  hash_data(nonce, &mut hashed_nonce);

  debug!("{:?}", &hashed_nonce);

  // The nonce MUST be 24-bytes or 192-bits and unique
  let nonce = XNonce::from_slice(&hashed_nonce);

  // Finally encrypt the text
  let ciphertext = cipher.encrypt(nonce, data).expect("encryption failure!");

  debug!("{:?}", &ciphertext);

  // Return the encrypted text
  ciphertext
}

/// This function takes a secret key, the nonce and some data and gives back the original data
pub fn decrypt_data(key: &[u8], nonce: &[u8], data: &[u8]) -> Result<Vec<u8>, CryptoErrors> {
  debug!("Decrypting: {:?} with {:?}", &data, &key);

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
  hash_data(nonce, &mut hashed_nonce);

  debug!("{:?}", &hashed_nonce);

  // The nonce MUST be 24-bytes or 192-bits and unique
  let nonce = XNonce::from_slice(&hashed_nonce);

  cipher
    .decrypt(nonce, data)
    .map_err(|_| CryptoErrors::DecryptionError)
}

// Makes an encrypted binary BSON field from `bytes`
// pub fn make_encrypted_binary_bson(bytes: Vec<u8>) -> Binary {
//   Binary {
//     bytes,
//     subtype: Encrypted,
//   }
// }

/// Encrypt and returns an encrypted bson binary
pub fn encrypt_bson_binary(key: &[u8], nonce: &[u8], data: &[u8]) -> Binary {
  let bytes = encrypt_data(key, nonce, data);
  Binary {
    bytes,
    subtype: Encrypted,
  }
}

/// Decrypts a encrypted BSON binary
pub fn decrypt_bson_binary(
  key: &[u8],
  nonce: &[u8],
  data: Binary,
) -> Result<Vec<u8>, CryptoErrors> {
  let bytes = data.bytes;
  decrypt_data(key, nonce, &bytes)
}

/// Decrypts an optional BSON binary
pub fn decrypt_optional_bson_binary(
  key: &[u8],
  nonce: &[u8],
  data: Option<Binary>,
) -> Option<Result<Vec<u8>, CryptoErrors>> {
  if let Some(data) = data {
    Some(decrypt_bson_binary(key, nonce, data))
  } else {
    None
  }
}
