use std::error::Error;

use base64::prelude::{Engine as _, BASE64_STANDARD};
use sha2::{Sha256, Digest};

use crate::datablock::DataBlock;
use crate::mining;

pub trait Block:ToString {
    fn data_hash(&self) -> String;
    fn block_data(&self) -> String {
        self.to_string()
    }
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
    block_data: DataBlock,
    old_block_hash: String,
}

pub struct MinedBlock {
    header: String,
    pub block_data: DataBlock,
    block_hash: String,
    old_block_hash: String,
}

#[derive(Debug)]
pub struct GenesisBlock {
    header: String,
    block_data: DataBlock,
    block_hash: String,
}

impl GenesisBlock {
    pub fn new(block_padding:String, data:&DataBlock) -> GenesisBlock {
        let block_hash = Sha256::digest(data.to_string()+&block_padding);
        GenesisBlock {
            header: "header text".to_owned(),
            block_data: data.clone(),
            block_hash: BASE64_STANDARD.encode(&block_hash),
        }
    }
    pub fn from_datablock(block_padding:String, data:DataBlock) -> GenesisBlock {
        let block_hash = Sha256::digest(data.to_string()+&block_padding);
        GenesisBlock {
            header: "header text".to_owned(),
            block_data: data,
            block_hash: BASE64_STANDARD.encode(&block_hash),
        }
    }
}

impl ToString for GenesisBlock {
    fn to_string(&self) -> String {
        self.block_data.to_string()
    }
}

impl ToString for BaseBlock {
    fn to_string(&self) -> String {
        self.block_data.to_string()
    }
}

impl Block for GenesisBlock {
    fn data_hash(&self) -> String {
        self.block_data.hash()
    }
}

impl Block for BaseBlock {
    fn data_hash(&self) -> String {
        self.block_data.hash()
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

impl BaseBlock {
    pub fn new(old_block_hash:String, data:&DataBlock) -> BaseBlock {
        BaseBlock {
            header: "header text".to_owned(),
            block_data: data.clone(),
            old_block_hash,
        }
    }
    pub fn upgrade(self) -> Result<MinedBlock,Box<dyn Error>> {
        match mining::verify_block_hash(&self) {
            Ok(block_hash) => {
                Ok(MinedBlock {
                    header:self.header, 
                    block_data: self.block_data,  
                    block_hash:block_hash, 
                    old_block_hash: self.old_block_hash,
                })
            },
            Err(e) => Err(e)
        }
        
    }
}