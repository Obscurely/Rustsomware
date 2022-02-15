use rand::prelude::*;
use rand_hc::Hc128Rng;

pub struct KeyGen {
    rng: Hc128Rng,
}

impl KeyGen {
    pub fn from(rng: Hc128Rng) -> KeyGen {
        KeyGen { rng }
    }

    pub fn gen_key_bytes(&mut self) -> [u8; 32] {
        let mut key_bytes: [u8; 32] = [0; 32];
        self.rng.fill_bytes(&mut key_bytes);
        key_bytes
    }

    pub fn gen_nonce_bytes(&mut self) -> [u8; 12] {
        let mut nonce_bytes: [u8; 12] = [0; 12];
        self.rng.fill_bytes(&mut nonce_bytes);
        nonce_bytes
    }
}
