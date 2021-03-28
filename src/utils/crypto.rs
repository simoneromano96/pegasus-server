use chacha20poly1305::{
  aead::{Aead, NewAead},
  Key, XChaCha20Poly1305, XNonce,
};
use log::debug;
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha3::{Digest, Sha3_256};

pub fn encrypt_data(key: String, data: String) -> Vec<u8> {
  debug!("Encrypting: {:?} with {:?}", &data, &key);

  // Create a SHA3-256 object
  let mut hasher = Sha3_256::new();

  // Write input message
  hasher.update(key.as_bytes());

  // Read hash digest
  let hashed_key = hasher.finalize();

  debug!("{:?}", &hashed_key);

  // The key MUST be 32-bytes or 256-bits, this is why we hash it
  let key = Key::from_slice(&hashed_key);

  // Create the cipher with the given key
  let cipher = XChaCha20Poly1305::new(key);

  // Generate a random nonce getting entropy from the OS
  let mut rng = ChaCha20Rng::from_entropy();
  let random_nonce: [u8; 24] = rng.gen();

  debug!("{:?}", &random_nonce);

  // The MUST be 24-bytes or 192-bits and unique
  let nonce = XNonce::from_slice(&random_nonce);

  // Finally encrypt the text
  let ciphertext = cipher
    .encrypt(nonce, data.as_bytes())
    .expect("encryption failure!");

  debug!("{:?}", &ciphertext);

  ciphertext
}

pub fn decrypt_data() {}
