use std::error::Error;

use rsa::{RsaPrivateKey,RsaPublicKey};
use rsa::pkcs8::{EncodePublicKey, EncodePrivateKey,LineEnding};

pub fn gen_pkcs8_pair(rng:&mut rand::rngs::ThreadRng, bits:usize) -> Result<(String, String), Box<dyn Error>> {
    let priv_key = RsaPrivateKey::new(rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    Ok((pub_key.to_public_key_pem(LineEnding::LF)?,priv_key.to_pkcs8_pem(LineEnding::LF)?.to_string()))
}

pub fn gen_pkcs8_batch(num: usize) -> Result<(Vec<String>,Vec<String>),Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let mut public_keys = Vec::with_capacity(num);
    let mut private_keys = Vec::with_capacity(num);
    for _ in 0..num {
        let (publ, pri) = gen_pkcs8_pair(&mut rng, bits)?;
        public_keys.push(publ);
        private_keys.push(pri);
    }
    Ok((public_keys,private_keys))
}