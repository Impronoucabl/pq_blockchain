mod block;
mod chain;

fn main() {
    let mut start = chain::Handler::new("hello, World!".to_owned());
    let block2 = block::BaseBlock::new("".to_string(), "test block hash".to_owned(), "data2".to_owned());
    start = start.add(block2).expect("still testing");
    println!("Complete!");
}
