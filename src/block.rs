use std::error::Error;

use base64::prelude::{Engine as _, BASE64_STANDARD};
use rsa::{pkcs1v15::SigningKey, pkcs8::DecodePrivateKey, RsaPrivateKey, signature::Signer};
use sha2::{Sha256, Digest};

use crate::datablock::DataBlock;

pub trait Block:ToString {
    fn data_hash(&self) -> String;
    fn block_data(&self) -> &DataBlock;
}

pub trait Sealed {
    fn block_hash(&self) -> &str;
}

pub trait NewBlock {
    fn old_block_hash(&self) -> &str;
}
pub trait Signed {
    fn signature(&self) -> &str;
}

#[derive(Debug)]
pub struct BaseBlock {
    header: String,
    block_data: DataBlock,
    old_block_hash: String,
}
#[derive(Debug)]
pub struct SealedBlock {
    inner_block: BaseBlock,
    block_hash: String,
}

#[derive(Debug)]
pub struct SignedBlock {
    inner_block: SealedBlock,
    sig_str: String,
}

#[derive(Debug)]
pub struct GenesisBlock {
    block_data: DataBlock,
    block_hash: String,
    sig_str: String,
}

impl ToString for GenesisBlock {
    fn to_string(&self) -> String {
        self.block_data.to_string()
    }
}

impl ToString for BaseBlock {
    fn to_string(&self) -> String {
        self.block_data.to_string() + self.old_block_hash() + &self.header 
    }
}

impl ToString for SealedBlock {
    fn to_string(&self) -> String {
        self.inner_block.to_string() + &self.block_hash
    }
}

impl ToString for SignedBlock {
    fn to_string(&self) -> String {
        self.inner_block.to_string() + &self.sig_str
    }
}

macro_rules! impl_block {
    ($block_type:ty) => {
        impl Block for $block_type {
            fn data_hash(&self) -> String {
                self.block_data.hash()
            }
            fn block_data(&self) -> &DataBlock {
                &self.block_data
            }
        } 
    }
}

macro_rules! impl_newblock {
    ($block_type:ty) => {
        impl NewBlock for $block_type {
            fn old_block_hash(&self) -> &str {
                &self.inner_block.old_block_hash()
            }
        }
    };
}

macro_rules! impl_sealed {
    ($block_type:ty) => {
        impl Sealed for $block_type {
            fn block_hash(&self) -> &str {
                &self.block_hash
            }
        }
    };
}

macro_rules! impl_inner_block {
    ($block_type:ty) => {
        impl Block for $block_type {
            fn data_hash(&self) -> String {
                self.inner_block.data_hash()
            }
            fn block_data(&self) -> &DataBlock {
                self.inner_block.block_data()
            }
        }
    };
}

impl_block!(GenesisBlock);
impl_block!(BaseBlock);
impl_inner_block!(SealedBlock);
impl_inner_block!(SignedBlock);

impl_newblock!(SealedBlock);
impl_newblock!(SignedBlock);

impl_sealed!(SealedBlock);
impl_sealed!(GenesisBlock);

impl NewBlock for BaseBlock {
    fn old_block_hash(&self) -> &str {
        &self.old_block_hash
    }
}

impl GenesisBlock {
    pub fn new(data:&DataBlock, signer:&str) -> Result<GenesisBlock, Box<dyn Error>> {
        let data_hash = Sha256::digest(data.to_string());
        let block_hash = BASE64_STANDARD.encode(&data_hash);
        let private_key = RsaPrivateKey::from_pkcs8_pem(signer)?;
        let signing_key = SigningKey::<sha2::Sha256>::new_unprefixed(private_key);
        let sig_str = signing_key.sign(block_hash.as_bytes()).to_string();
        Ok(GenesisBlock {
            block_data: data.clone(),
            block_hash,
            sig_str,
        })
    }
}

impl BaseBlock {
    pub fn new(old_block_hash:String, data:&DataBlock) -> BaseBlock {
        BaseBlock {
            header: "header text".to_owned(),
            block_data: data.clone(),
            old_block_hash,
        }
    }
}

impl From<BaseBlock> for SealedBlock {
    fn from(base_block: BaseBlock) -> Self {
        let block_hash = Sha256::digest(base_block.to_string());
        SealedBlock {
            inner_block: base_block,
            block_hash: BASE64_STANDARD.encode(block_hash), 
        }
    }
}

impl SignedBlock {
    pub fn from(sealed_block: SealedBlock, signer:&str) -> Result<SignedBlock,Box<dyn Error>> {
        let private_key = RsaPrivateKey::from_pkcs8_pem(signer)?;
        let signing_key = SigningKey::<sha2::Sha256>::new_unprefixed(private_key);
        let sig_str = signing_key.sign(sealed_block.block_hash().as_bytes()).to_string();
        Ok(SignedBlock {
            inner_block: sealed_block,
            sig_str, 
        })
    }
}