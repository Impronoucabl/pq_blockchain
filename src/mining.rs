use std::{error::Error, fmt::{Debug, Display}};

use base64::prelude::{Engine as _, BASE64_STANDARD};
use sha2::{Digest, Sha256};

use crate::block::{ BaseBlock, Block};

pub fn verify_block_hash(block:&BaseBlock) -> Result<String, Box<dyn Error>> {
    let full_block = block.block_data();
    let hash = Sha256::digest(full_block);
    Ok(todo!())
}
#[derive(Debug)]
pub struct Miss {}

impl Error for Miss {}
impl Display for Miss {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hash did not meet target")
    }
}