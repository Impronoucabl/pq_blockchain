use base64::prelude::{Engine as _, BASE64_STANDARD_NO_PAD};
use sha2::{Sha512, Digest};

pub trait Block {
    fn hash(&self) -> &str;
    fn block_hash(&self) -> &str;
    fn data(&self) -> &str;
}

#[derive(Debug)]
pub struct BaseBlock {
    header: String,
    kem_key: String,
    sig_key: String,
    hash: String,
    block_hash: String,
    old_block_hash: String,
}

#[derive(Debug)]
pub struct GenesisBlock {
    header: String,
    kem_key: String,
    sig_key: String,
    hash: String,
    block_hash: String,
}

impl GenesisBlock {
    pub fn new(block_hash:String, data:String) -> GenesisBlock {
        let hash512 = Sha512::digest(data);
        GenesisBlock {
            header: "header text".to_owned(),
            kem_key: "Pub KEM Key (A & t)".to_owned(),
            sig_key: "Pub Sig Key".to_owned(),
            hash:  BASE64_STANDARD_NO_PAD.encode(&hash512),
            block_hash,
        }
    }
}

impl Block for GenesisBlock {
    fn hash(&self) -> &str {
        &self.hash
    }
    
    fn block_hash(&self) -> &str {
        &self.block_hash
    }
    
    fn data(&self) -> &str {
        todo!()
    }
}

impl BaseBlock {
    pub fn new(old_block_hash:String, block_hash:String, data:String) -> BaseBlock {
        let hash512 = Sha512::digest(data);
        BaseBlock {
            header: "header text".to_owned(),
            kem_key: "Pub KEM Key (A & t)".to_owned(),
            sig_key: "Pub Sig Key".to_owned(),
            hash:  BASE64_STANDARD_NO_PAD.encode(&hash512),
            block_hash,
            old_block_hash,
        }
    }
    pub fn old_block_hash(&self) -> &str {
        &self.old_block_hash
    }
}