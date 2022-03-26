use blockchainlib::*;
fn main() {
    let block = Block::new(0,now(), vec![0;32], 0, "Genesis block".to_owned());
    println!("{:?}", &block);

    let h = block.hash();
    println!("{:?}", &h);
}
