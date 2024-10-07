use std::{error::Error, fmt::{Debug, Display}};

use base64::prelude::{Engine as _, BASE64_STANDARD};
use sha2::{Digest, Sha256};

use crate::block::{self, BaseBlock, Block};

impl block::BaseBlock {
    fn swing(&self, nonce:u128) -> Result<String, Box<dyn Error>> {
        let mut split = Vec::with_capacity(16);
        for i in 0..16 {
            split.push((nonce >> i*8) as u8)
        }
        let padding = String::from_utf8(split)?;
        match verify_block_hash(self, &padding) {
            Ok(_) => Ok(padding),
            Err(e) => Err(e) 
        }
    }
}

pub fn mine(block:&BaseBlock) -> String {
    for i in 0..=u128::MAX {
        match block.swing(i) {
            Ok(padding) => return padding,
            Err(_) => continue
        }
    }
    "Failed to mine".to_owned()
}

pub fn verify_block_hash(block:&BaseBlock,block_padding:&str) -> Result<String, Box<dyn Error>> {
    let full_block = block.block_data().to_owned() + &block_padding;
    let hash = Sha256::digest(full_block);
    let mut difficulty = 2;
    for digits in hash.as_slice() {
        if difficulty <= 0 {
            break;
        }
        if digits > &0 {
            return Err(Box::new(Miss{}))
        }
        difficulty -= 1;
    }
    Ok(BASE64_STANDARD.encode(hash))
}
#[derive(Debug)]
pub struct Miss {}

impl Error for Miss {}
impl Display for Miss {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hash did not meet target")
    }
}