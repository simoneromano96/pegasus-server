use chacha20poly1305::{
  aead::{Aead, NewAead},
  Key, XChaCha20Poly1305, XNonce,
};
use log::debug;
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

pub fn encrypt_data(key: String, data: String) -> Vec<u8> {
  debug!("Encrypting: {:?} with {:?}", &data, &key);

  let key = Key::from_slice(key.as_bytes()); // 32-bytes

  let cipher = XChaCha20Poly1305::new(key);

  let mut rng = ChaCha20Rng::from_entropy();
  let random_nonce: [u8; 24] = rng.gen();

  debug!("{:?}", &random_nonce);

  let nonce = XNonce::from_slice(&random_nonce); // 24-bytes; unique

  let ciphertext = cipher
    .encrypt(nonce, data.as_bytes())
    .expect("encryption failure!");

  debug!("{:?}", &ciphertext);

  // let encrypted_text: String = String::from_utf8(ciphertext.clone()).expect("Invalid input");
  // debug!("{:?}", &encrypted_text);

  ciphertext
}

pub fn decrypt_data() {}
