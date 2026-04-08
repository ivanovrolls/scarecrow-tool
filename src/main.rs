use clap::{Parser, Subcommand};

#[derive(Parser)] //automatically generate code to parse command line arguments, Rust writes boilerplate for me :p
#[command(name = "crow")]
#[command(about = "Scarecrow is a CLI tool for managing development environments 🐦‍⬛", long_about = None)]
struct Cli { //defines my command line interface, the struct will hold the parsed arguments
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)] //automatically generate code to parse subcommands
enum Commands {
    Guard { //install and switch to env
        tool_version: String, //version of tool to use e.g. node 18.16.0
    },
    Pick { //change env
        tool_version: String, //version of tool to use e.g. node 18.16.0
    },
    Scare { //remove env
        tool_version: String, //version of tool to remove e.g. node 18.16.0
    },
    All, //list all envs
}

fn main() {
    let cli = Cli::parse();
match cli.command {
        Commands::Guard { tool_version } => {
            println!("🐦‍⬛ Guarding environment {}", tool_version);
        }
        Commands::Pick { tool_version } => {
            println!("🐦‍⬛ Picking environment {}", tool_version);
        }
        Commands::Scare { tool_version } => {
            println!("🐦‍⬛ Scaring away {}", tool_version);
        }
        Commands::All => {
            println!("🐦‍⬛ All installed versions:");
        }
    }
}
