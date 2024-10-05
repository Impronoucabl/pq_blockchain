use std::error::Error;

use base64::prelude::{Engine as _, BASE64_STANDARD_NO_PAD};
use sha2::{Sha512, Digest};

use crate::mining;

pub trait Block {
    fn hash(&self) -> &str;
    fn data(&self) -> &str;
}

pub trait Mined {
    fn block_hash(&self) -> &str;
}

pub trait NewBlock {
    fn old_block_hash(&self) -> &str;
}

#[derive(Debug)]
pub struct BaseBlock {
    header: String,
    kem_key: String,
    sig_key: String,
    hash: String,
    old_block_hash: String,
}

pub struct MinedBlock {
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
    fn data(&self) -> &str {
        todo!()
    }
}

impl Block for BaseBlock {
    fn hash(&self) -> &str {
        &self.hash
    }
    fn data(&self) -> &str {
        todo!()
    }
}

impl NewBlock for BaseBlock {
    fn old_block_hash(&self) -> &str {
        &self.old_block_hash
    }
}

impl NewBlock for MinedBlock {
    fn old_block_hash(&self) -> &str {
        &self.old_block_hash
    }
}

impl Mined for MinedBlock {
    fn block_hash(&self) -> &str {
        &self.block_hash
    }
}

impl Mined for GenesisBlock {
    fn block_hash(&self) -> &str {
        &self.block_hash
    }
}

impl BaseBlock {
    pub fn new(old_block_hash:String, data:String) -> BaseBlock {
        let hash512 = Sha512::digest(data);
        BaseBlock {
            header: "header text".to_owned(),
            kem_key: "Pub KEM Key (A & t)".to_owned(),
            sig_key: "Pub Sig Key".to_owned(),
            hash:  BASE64_STANDARD_NO_PAD.encode(&hash512),
            old_block_hash,
        }
    }
    pub fn old_block_hash(&self) -> &str {
        &self.old_block_hash
    }
    pub fn upgrade(self, block_hash:&str) -> Result<MinedBlock,Box<dyn Error>> {
        match mining::verify_block_hash(&self, block_hash) {
            Ok(_) => Ok(MinedBlock { 
                header:self.header, 
                kem_key: self.kem_key, 
                sig_key: self.sig_key, 
                hash:self.hash, 
                block_hash: block_hash.to_string(), 
                old_block_hash: self.old_block_hash
            }),
            Err(e) => Err(e)
        }
        
    }
}