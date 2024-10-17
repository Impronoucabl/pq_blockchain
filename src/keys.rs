use std::error::Error;

use rsa::pkcs1v15::{Signature, SigningKey, VerifyingKey};
use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::signature::{SignerMut, Verifier};
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding};
use sha2::Sha256;

use crate::chain;

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

pub fn verify_signature(data:&str, sig:&str, pub_key:&str) -> Result<(),Box<dyn Error>> {
    let public_key = RsaPublicKey::from_public_key_pem(pub_key)?;
    let verifying_key = VerifyingKey::<Sha256>::new_unprefixed(public_key);
    let mut buff:String = "".to_string();
    let mut full = false;
    let mut hex_vec = Vec::new();
    for n in sig.chars() {
        if full {
            buff.push(n);
            hex_vec.push(u8::from_str_radix(&buff, 16).expect(&("bad char was:".to_string() + &buff)))
        } else {
            buff = String::from(n);
        }
        full = !full;
    };
    match verifying_key.verify(data.as_bytes(),&Signature::try_from(hex_vec.as_slice())?) {
        Ok(_) => Ok(()),
        Err(_) => Err(Box::new(chain::BadSigError{}))
    }
}

pub fn test() -> Result<(),Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let (publ,pri) = gen_pkcs8_pair(&mut rng, bits)?;
    let data = "jhfksjdhfskdjfhdskhsdjkfjh".to_string();
    let private_key = RsaPrivateKey::from_pkcs8_pem(&pri)?;
    let mut signing_key = SigningKey::<sha2::Sha256>::new_unprefixed(private_key);
    let sig_str = signing_key.sign(&data.as_bytes());
    let public_key = RsaPublicKey::from_public_key_pem(&publ)?;
    let verifying_key = VerifyingKey::<Sha256>::new_unprefixed(public_key);
    let sig1 = sig_str.to_string();


    let mut buff:String = "".to_string();
    let mut full = false;
    let mut new_vec = Vec::new();
    for n in sig1.chars() {
        if full {
            buff.push(n);
            new_vec.push(u8::from_str_radix(&buff, 16).unwrap())
        } else {
            buff = String::from(n);
        }
        full = !full;
    };
    match verifying_key.verify(data.as_bytes(), &Signature::try_from(new_vec.as_slice())?) {
        Ok(_) => Ok(()),
        Err(_) => Err(Box::new(chain::BadSigError{}))
    }
}