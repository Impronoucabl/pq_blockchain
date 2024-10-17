use std::error::Error;

use crate::block::{Block, GenesisBlock, NewBlock, Sealed, SignedBlock, Verifiable};
use crate::datablock::{self, DataBlock, KeyBind};
use crate::keys;

pub struct Handler {
    root_block: GenesisBlock,
    chain: Vec<SignedBlock>,
    latest_hash: String,
}

impl Handler {
    pub fn new(data:&DataBlock, signer:&str) -> Result<Handler, Box<dyn Error>> {
        let start = GenesisBlock::new(data, signer)?;
        Ok(Handler { latest_hash: start.data_hash().to_string(), root_block: start, chain: Vec::new() })
    }
    pub fn latest_hash(&self) -> &str {
        &self.latest_hash
    }
    pub fn add(&mut self, block:SignedBlock) -> Result<(), Box<dyn Error>> {
        self.verify_next_block(&block)?;
        self.verify_hash(&block)?;
        self.verify_sig(&block)?;
        self.latest_hash = block.block_hash().to_owned();
        self.chain.push(block);
        Ok(())
    }
    fn verify_next_block(&self, block:&SignedBlock) -> Result<(),Box<dyn Error>> {
        if self.latest_hash == block.old_block_hash() {
            Ok(())
        } else {
            Err(Box::new(RaceBlockError{}))
        }
    }
    fn verify_hash(&self, block:&SignedBlock) -> Result<(),Box<dyn Error>> {
        match block.block_hash() == block.calc_hash() {
            true => Ok(()),
            false => Err(Box::new(CorruptBlock{})),
        }
    }
    fn verify_sig(&self, block: &SignedBlock) -> Result<(),Box<dyn Error>> {
        let mut good_sig = false;
        for nodes in self.get_node_list() {
            match keys::verify_signature(&block.block_hash(), block.sig(), nodes.key()) {
                Ok(_) => {
                    good_sig = true;
                    println!("Verified!");
                    break
                },
                Err(_) => {
                    println!("Sig no-good");
                    continue
                }
            }
        }
        if !good_sig {return Err(Box::new(BadSigError{}))};
        Ok(())
    }
    fn _keybind_vec(&self) -> Vec<(&KeyBind, usize)>{
        let rec_iter:Vec<(&KeyBind,usize)> = self.chain.iter().enumerate()
            .map(|(n,b)|b.block_data().data_ref().iter().zip([n+1;datablock::DATA_ENTRY_LIMIT as usize])).flatten().collect();
        let origin = self.root_block.block_data().data_ref().iter().zip([0;datablock::DATA_ENTRY_LIMIT as usize]);
        origin.chain(rec_iter.into_iter()).collect()
    }
    pub fn get_full_ident_list(&self) -> Vec<(&str, usize)> {
        let mut list = Vec::new();
        for (bind, n) in self._keybind_vec() {
            list.push((bind.identity(),n))
        }          
        list
    }
    pub fn get_node_list(&self) -> Vec<&KeyBind> {
        let mut list = Vec::new();
        for (bind, _) in self._keybind_vec() {
            if bind.role() == "NODE" {
                list.push(bind)
            }
        }          
        list
    }
    pub fn chain(&self) -> &Vec<SignedBlock>{
        &self.chain
    }
}

#[derive(Debug)]
pub struct BadSigError {}
#[derive(Debug)]
struct RaceBlockError {}
#[derive(Debug)]
struct CorruptBlock {}
impl Error for BadSigError {}
impl Error for RaceBlockError {}
impl Error for CorruptBlock {}
impl std::fmt::Display for BadSigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bad Signature")
    }
}
impl std::fmt::Display for RaceBlockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A newer block has been added!")
    }
}
impl std::fmt::Display for CorruptBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stored & calc hashes don't match!")
    }
}