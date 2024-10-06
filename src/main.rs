use std::error::Error;

use block::Mined;
use datablock::{DataBlock, KeyBind};

mod datablock;
mod block;
mod mining;
mod chain;

fn main() -> Result<(),Box<dyn Error>>{
    let key1 = "------begin public key------
    MII+82y4rfbewhbf82hrh8ehw8ydgfw8egfw8eyf
    gw8dygfw8edfygwe8fygwyd8fygwdEREFFHH568g
    fydgwyf8wewijdoaksoQWEG345akjkjjkjcvcrfo
    ------end public key------".to_owned();
    let key2 = "------begin public key------
    MII+82y4rfbdata2plkpohktijsdbhweyncvRTHG
    FopvcoitrjensdkawosxcoijujdDGYFdjfh76S9d
    ------end public key------".to_owned();
    let key3 = "------begin public key------
    MII+coitrjensdkawosxcoijujdDGYFdjfh76S9d
    82y4rfbdata2plkpohktijsdbhweyncvRTHG1034
    FopvPLZMjnfd8h49wkasdfnm1lmq1sdf356dFGdo
    ------end public key------".to_owned();
    let key4 = "------begin public key------
    MII+egfw8eyf8fygwyd8qwrxXx42069xXxaoskdP
    gw8dygfw8edfygwefygwdEREFFHH568gBOOBB135
    fydgwyf8wewijdoaksoQWEG345akjkjjkjoEDT54
    ------end public key------".to_owned();
    let bind1 = KeyBind::new(key1,"Alice".to_owned(), "abc".to_owned());
    let bind2 = KeyBind::new(key2,"Bob".to_owned(), "afghgghbc".to_owned());
    let bind3 = KeyBind::new(key3,"PKIRCA1".to_owned(), "98sud98fSDN".to_owned());
    let bind4 = KeyBind::new(key4,"PKIRCA2".to_owned(), "drtdtd".to_owned());
    let mut bind_vec1 = Vec::with_capacity(2);
    let mut bind_vec2 = Vec::with_capacity(2);
    bind_vec1.push(bind3);
    bind_vec1.push(bind4);
    bind_vec2.push(bind1);
    bind_vec2.push(bind2);
    let data1 = DataBlock::new(bind_vec1 );
    let data2 = DataBlock::new(bind_vec2);
    //let data3 = DataBlock::new(key3, "PKICA".to_owned());
    let mut start = chain::Handler::new(data1);
    
    start = add_data(start, data2);
    //start = add_data(start, data3);
    println!("Complete!");
    Ok(())
}

fn add_data(h:chain::Handler, data:DataBlock) -> chain::Handler {
    let block2 = block::BaseBlock::new(h.latest_hash().to_string(), data);
    let new_pad = mining::mine(&block2);
    println!("{}", &new_pad);
    let new_block = block2.upgrade(&new_pad).expect("still testing");
    println!("{}",new_block.block_hash());
    h.add(new_block).expect("We mined this ourselves")
}
