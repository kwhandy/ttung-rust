use clap::Args;

#[derive(Args)]
pub struct UpcaseArgs {
    /// The text to transform
    pub text: String,
}

pub fn run(args: UpcaseArgs) {
    // In Ruby: text.upcase!
    // In Rust: We must create a 'mut' (mutable) variable to change it.
    let mut transformed = args.text; 
    
    // We overwrite 'transformed' with the uppercase version.
    transformed = transformed.to_uppercase();

    println!("Result: {}", transformed);
}

