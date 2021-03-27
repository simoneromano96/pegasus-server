use chacha20poly1305::{
  aead::{Aead, NewAead},
  Key, XChaCha20Poly1305, XNonce,
};
use log::debug;
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

pub fn encrypt_data(key: String, data: String) -> Vec<u8> {
  debug!("Encrypting: {:?} with {:?}", &data, &key);

  // MUST be 32-bytes
  let key = Key::from_slice(key.as_bytes());

  let cipher = XChaCha20Poly1305::new(key);

  let mut rng = ChaCha20Rng::from_entropy();
  let random_nonce: [u8; 24] = rng.gen();

  debug!("{:?}", &random_nonce);

   // MUST be 24-bytes; unique
  let nonce = XNonce::from_slice(&random_nonce);

  let ciphertext = cipher
    .encrypt(nonce, data.as_bytes())
    .expect("encryption failure!");

  debug!("{:?}", &ciphertext);

  ciphertext
}

pub fn decrypt_data() {}
