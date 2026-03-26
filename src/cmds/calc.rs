use clap::Args;

#[derive(Args)]
pub struct CalcArgs {
    /// First number
    pub a: i32, // i32 is a 32-bit Integer (Ruby's Integer)
    /// Second number
    pub b: i32,
}

pub fn run(args: CalcArgs) {
    // In Rust, math is extremely fast because types are known at compile time.
    let sum = args.a + args.b;
    let product = args.a * args.b;

    println!("{} + {} = {}", args.a, args.b, sum);
    println!("{} * {} = {}", args.a, args.b, product);
}

