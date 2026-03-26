// 'use' is like 'require' in Ruby. 
// We bring in 'Parser' (the CLI builder) and 'Subcommand' (the router).
use clap::{Parser, Subcommand};

// 'mod' tells Rust: "Go find a folder named 'cmds' with a 'mod.rs' inside."
// Similar to: require_relative 'cmds/mod'
mod cmds;

#[derive(Parser)]
#[command(name = "ttung", version = "0.1.0", about = "Ruby-to-Rust learning tool")]
struct Cli {
    // This 'enum' defines our subcommands (like 'upcase', 'reverse').
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert text to UPPERCASE (Learning Mutability)
    Upcase(cmds::upcase::UpcaseArgs),
    /// Reverse a string (Learning Ownership)
    Reverse(cmds::reverse::ReverseArgs),
    /// Simple math (Learning Numbers)
    Calc(cmds::calc::CalcArgs)
}

fn main() {
    let cli = Cli::parse();

    // 'match' is like Ruby's 'case/when', but BETTER.
    // If you add a command to the Enum but forget it here, it WON'T compile.
    match cli.command {
        Commands::Upcase(args) => cmds::upcase::run(args),
        Commands::Reverse(args) => cmds::reverse::run(args),
        Commands::Calc(args) => cmds::calc::run(args),
    }
}

