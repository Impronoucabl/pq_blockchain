use std::error::Error;

use block::{GenesisBlock, Sealed, SealedBlock, Signed, SignedBlock};
use datablock::{DataBlock, KeyBind};
use keys::gen_pkcs8_batch;

mod keys;
mod datablock;
mod block;
mod chain;

const BIND_PER_BLOCK: [usize;3] = [3,2,1];
const TEST_NUM: usize = get_test_num(&BIND_PER_BLOCK);

const fn get_test_num(binding:&[usize]) -> usize {
    let len = binding.len();
    let mut count = 0;
    let mut num = 0;
    while count < len {
        num += binding[count];
        count += 1;
    }
    num
}

fn test_init() -> (Vec<DataBlock>, Vec<String>) {
    let (pub_keys,pri_keys) = gen_pkcs8_batch(TEST_NUM).expect("testing");
    let identities = Vec::from(["Alice","Bob","PKIRCA1","PKIRCA2","Eve","OCSP"]);
    let privledges = Vec::from(["NODE";TEST_NUM]);
    let mut pair_it = pub_keys.iter();
    let mut ident_it = identities.iter();
    let mut sig_it = privledges.iter();
    let mut block_vec = Vec::with_capacity(BIND_PER_BLOCK.len());
    for n in BIND_PER_BLOCK {
        let mut count = 0;
        let mut bind_vec = Vec::with_capacity(n);
        while count < n {
            let bind = KeyBind::new(
                pair_it.next().unwrap().to_string(),
                ident_it.next().unwrap().to_string(),
                sig_it.next().unwrap().to_string()
            );
            bind_vec.push(bind);
            count += 1;
        }
        block_vec.push(DataBlock::new(bind_vec));
    }
    (block_vec,pri_keys)
}

// fn main() -> Result<(),Box<dyn Error>> {
//     keys::test()
// }

fn main() -> Result<(),Box<dyn Error>>{
    let (block_vec, mut pri_keys) = test_init();
    pri_keys.reverse();
    let mut data = block_vec.iter();
    let signer1 = pri_keys.pop().expect("there should be 5 more");
    let signer2 = pri_keys.pop().expect("there should be 4 more");
    let mut start = chain::Handler::new(data.next().expect("We just built this"),&signer1)?; 
    //let block = start.chain().pop().unwrap();

    start = add_data(start, data.next().expect("We built this with more than 1 block"), &signer1);
    //start = add_data(start, data.next().expect("We built this with more than 2 blocks"), &signer2);
    println!("Complete!");
    Ok(())
}

fn add_data(h:chain::Handler, data:&DataBlock, signer:&str) -> chain::Handler {
    let block2 = block::BaseBlock::new(h.latest_hash().to_string(), data);
    let new_block: SealedBlock = block2.into();
    let signed_block = SignedBlock::from(new_block, signer);
    h.add(signed_block.expect("No signing errors here!")).expect("We mined this ourselves")
}
