use std::error::Error;

use crate::block::{self, Block};

pub struct Handler {
    root_block: block::GenesisBlock,
    chain: Vec<block::BaseBlock>,
    latest_hash: String,
}

impl Handler {
    pub fn new(data:String) -> Handler {
        let start = block::GenesisBlock::new("".to_string(), data);
        Handler { latest_hash: start.block_hash().to_string(), root_block: start, chain: Vec::new() }
    }
    pub fn add(mut self, block:block::BaseBlock) -> Result<Self, Box<dyn Error>> {
        self.verify_block(&block)?;
        self.chain.push(block);
        Ok(self)
    }
    fn verify_block(&self, block:&block::BaseBlock) -> Result<(),Box<dyn Error>> {
        if self.latest_hash == block.old_block_hash() {
            Ok(())
        } else {
            Err(todo!())
        }
    }
}