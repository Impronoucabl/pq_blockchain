use std::error::Error;

use crate::{block::{self, Block, Mined, NewBlock}, datablock::DataBlock};

pub struct Handler {
    root_block: block::GenesisBlock,
    chain: Vec<block::MinedBlock>,
    latest_hash: String,
}

impl Handler {
    pub fn new(data:DataBlock) -> Handler {
        let start = block::GenesisBlock::new("".to_string(), data);
        Handler { latest_hash: start.data_hash().to_string(), root_block: start, chain: Vec::new() }
    }
    pub fn latest_hash(&self) -> &str {
        &self.latest_hash
    }
    pub fn add(mut self, block:block::MinedBlock) -> Result<Self, Box<dyn Error>> {
        self.verify_next_block(&block)?;
        self.latest_hash = block.block_hash().to_owned();
        self.chain.push(block);
        Ok(self)
    }
    fn verify_next_block(&self, block:&block::MinedBlock) -> Result<(),Box<dyn Error>> {
        if self.latest_hash == block.old_block_hash() {
            Ok(())
        } else {
            Err(todo!())
        }
    }
}