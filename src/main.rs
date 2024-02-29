use std::path::PathBuf;
use rand::Rng;
use clap::{Parser, Subcommand};
use std::process::Command; // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use scan_fmt::scan_fmt;

#[derive(Parser)]
#[command(name = "notus", about="Notes for us", long_about = "A DND notes app with insane functionality (dungeon generation, dice rolling, markdown support, exporting)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    // /// Sets a custom output file
    // #[arg(short, long, value_name = "FILE")]
    // output: Option<PathBuf>,

    // /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Rolls a die
    Roll {
        #[arg()]
        input: String,
    }
}
fn main() {
//initalize cli and Random number generator
    let cli = Cli::parse();
    let mut rng = rand::thread_rng();
    match &cli.command {
        Commands::Roll { input } => {
            let (num, die) = scan_fmt!(input, "{}d{}", i32, i32).unwrap();
            roll(num, die, &mut rng)
        }
    }
}  

fn roll(num: i32, die: i32, rng: &mut rand::rngs::ThreadRng){
    println!("Rolling {}d{}:", num, die);
    let mut roll = 0;
    for i in 0..num{
        let rolled: i32 = rng.gen_range(1..=die);
        println!("Roll {}: {}",i+1, rolled);
        roll+= rolled;
    }
    println!("Total: {}", roll);
}

#[test]
fn test_roll_command() {
    let mut cmd = Command::cargo_bin("notus").unwrap();
    cmd.arg("roll").arg("2d8");
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("Rolling 2d8:"));
}