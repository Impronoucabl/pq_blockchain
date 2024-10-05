use std::error::Error;

use block::Mined;

mod block;
mod mining;
mod chain;

fn main() -> Result<(),Box<dyn Error>>{
    let mut start = chain::Handler::new("hello, World!".to_owned());
    let block2 = block::BaseBlock::new(start.latest_hash().to_string(), "data2".to_owned());
    let new_pad = mining::mine(&block2);
    println!("{}", &new_pad);
    let new_block = block2.upgrade(&new_pad).expect("still testing");
    println!("{}",new_block.block_hash());
    start = start.add(new_block)?;
    println!("Complete!");
    Ok(())
}
