use blockchainlib::*;

fn main() {
    print!("Hello, Blockchain!\n");

    // Create block object
    let block = Block::new(0, 0, vec![0; 32], 0, "First Block".to_owned());

    // Print first Block object
    // uses :? to get the Debug one | & means reference to block
    println!("{:?}", &block);

    // Generate hash for block
    let hash = block.hash();

    // Print hash - this is a vector so use Debug displayer to format it
    println!("First Block hashed: {:?}", &hash);
}