use std::error::Error;

use base64::prelude::{Engine as _, BASE64_STANDARD_NO_PAD};

use crate::block::{BaseBlock, Block};

const TARGET:u128 = 340282366920938463463374607431768211452;

pub fn verify_block_hash(block:&BaseBlock,block_hash:&str) -> Result<(), Box<dyn Error>> {
    println!();
    println!("{}",block.hash());
    let hash = block.hash().to_owned() + block_hash;
    println!("{}",hash);
    let hash_num = BASE64_STANDARD_NO_PAD.decode(hash)?;
    println!();
    for num in hash_num {
        print!("{}",num);
    }
    Ok(())
    //TODO
}