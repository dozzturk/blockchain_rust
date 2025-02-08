use blockchainlib::*;

fn main() {
    print!("Hello, Blockchain!\n");

    // Create block object
    let mut block = Block::new(0, 0, vec![0; 32], 0, "First Block".to_owned(), 0x000ffffffffffffffffffffffffffff); // note: 


    // OLD:
    // Print first Block object
    // uses :? to get the Debug one | & means reference to block
    // println!("{:?}", &block);

    // Generate hash for block
    // let hash_block = block.hash();

    // Print hash - this is a vector so use Debug displayer to format it
    //println!("First Block hashed: {:?}", &hash);

    //block.hash = hash_block;
    //println!("{:?}", &block); 

    // NEW:
    // MINING
    block.hash = block.hash();
    println!("{:?}", &block); 

    // block.mine();
    // println!("{:?}", &block);




}