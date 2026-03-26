use clap::Args;

#[derive(Args)]
pub struct ReverseArgs {
    pub text: String,
}

pub fn run(args: ReverseArgs) {
    // Rust strings are UTF-8. To reverse them properly, we:
    // 1. Get the characters (.chars())
    // 2. Flip them (.rev())
    // 3. Turn them back into a String (.collect())
    let flipped: String = args.text.chars().rev().collect();
    
    // Note: 'args.text' was "moved" into this logic.
    // Rust will automatically delete the memory for 'flipped' 
    // the moment this function ends. No Garbage Collector needed!
    println!("Flipped: {}", flipped);
}

