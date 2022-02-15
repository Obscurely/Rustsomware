use aes_gcm_siv::aead::{Aead, NewAead};
use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce};
use rand::prelude::*;
use rand_hc::Hc128Rng;
use std::str;

fn main() {
    let msg = "This is my encrypted text!";

    let mut rng = Hc128Rng::from_entropy();
    let mut key_bytes: [u8; 32] = [0; 32];
    let mut nonce_bytes: [u8; 12] = [0; 12];
    rng.fill_bytes(&mut key_bytes);
    rng.fill_bytes(&mut nonce_bytes);

    let key = Key::from_slice(&key_bytes);
    let cipher = Aes256GcmSiv::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let cipher_text = cipher.encrypt(&nonce, msg.as_ref()).expect("error");

    let plain_text = cipher.decrypt(&nonce, cipher_text.as_ref()).expect("error");

    for x in cipher_text {
        print!("{}", x);
    }

    println!("\n{}", str::from_utf8(&plain_text).expect("test"))
}
