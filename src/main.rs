use std::error::Error;

mod block;
mod mining;
mod chain;

fn main() -> Result<(),Box<dyn Error>>{
    let mut start = chain::Handler::new("hello, World!".to_owned());
    let block2 = block::BaseBlock::new("".to_string(), "data2".to_owned());
    start = start.add(block2.upgrade("sAseVV").expect("still testing"))?;
    println!("Complete!");
    Ok(())
}
