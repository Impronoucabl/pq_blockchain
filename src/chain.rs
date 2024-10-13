use std::error::Error;

use crate::block::{self, Block, Sealed, NewBlock};
use crate::datablock::DataBlock;

pub struct Handler {
    root_block: block::GenesisBlock,
    chain: Vec<block::SealedBlock>,
    latest_hash: String,
}

impl Handler {
    pub fn new(data:&DataBlock, signer:&str) -> Result<Handler, Box<dyn Error>> {
        let start = block::GenesisBlock::new(data, signer)?;
        Ok(Handler { latest_hash: start.data_hash().to_string(), root_block: start, chain: Vec::new() })
    }
    pub fn latest_hash(&self) -> &str {
        &self.latest_hash
    }
    pub fn add(mut self, block:block::SealedBlock) -> Result<Self, Box<dyn Error>> {
        self.verify_next_block(&block)?;
        self.latest_hash = block.block_hash().to_owned();
        self.chain.push(block);
        Ok(self)
    }
    fn verify_next_block(&self, block:&block::SealedBlock) -> Result<(),Box<dyn Error>> {
        if self.latest_hash == block.old_block_hash() {
            Ok(())
        } else {
            Err(todo!())
        }
    }
    pub fn get_full_ident_list(&self) -> Vec<(&str, usize)> {
        let mut list = Vec::new();
        for n in 0..self.chain.len() {
            let data = &self.chain[n].block_data(); 
            for ident in data.data_ref() {
                list.push((ident.identity(),n))
            }
        }
        list
    }
    pub fn get_ident_role_list<T>(&self, role:T) -> Vec<(&str, usize)> 
        where
            T: ToString,
    {
        let mut list = Vec::new();
        for n in 0..self.chain.len() {
            let data = &self.chain[n].block_data(); 
            for ident in data.data_ref() {
                if ident.role() == role.to_string() {
                    list.push((ident.identity(),n))
                }
            }
        }
        list
    }
}