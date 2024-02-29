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
    #[arg( required = false)]    
    note: Option<String>,

    #[command(subcommand)]
    command: Commands,
    

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
    if let Some(note) = cli.note {
        println!("{}", note);
    }
    else {
       match &cli.command {
        Commands::Roll { input } => {
            let (num, die) = scan_fmt!(
                input, "{}d{}", i32, i32).unwrap();
            roll(num, die, &mut rng)
        }
        _ =>{ println!("you need to write something man") }
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
    let mut rng = rand::thread_rng();
    let mut cmd = Command::cargo_bin("notus").unwrap();
    //randomize the roll
    let num = rng.gen_range(1..=6);
    let die = rng.gen_range(1..=20);
    cmd.arg("roll").arg(format!("{}d{}", num, die));
    cmd.assert()
       .success()
       .stdout(predicate::str::contains(format!("Rolling {}d{}:", num, die)));
}