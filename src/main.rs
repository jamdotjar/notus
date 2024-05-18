use std::{path::PathBuf, thread::AccessError};
use std::fs;
use commands::{handle_note, handle_quicknote, handle_roll};
use rand::Rng;
use std::hash::{Hash, DefaultHasher, Hasher};
use clap::{Parser, Subcommand};
use std::process::Command; // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use scan_fmt::scan_fmt;
use chrono::Utc;
use crate::notes::Note;

mod notes;
mod cli;
mod commands;

use cli::{Cli, Commands, NoteAction};
use notes::{save_notes_list, NoteID};

fn main() {
//initalize cli and Random number generator
    let cli = Cli::parse();
    let mut notes = load_notes_list();
    let mut active: Option<NoteID> = notes.iter().find(|n| n.active).cloned();

    //println!("{:?}", notes);
    match cli.command {
        Commands::Roll { input } => {
            handle_roll(&input);
        },
        Commands::Quick { input} => {
            handle_quicknote(&input, &active.unwrap())
        },
        Commands::Note { action } => {
            handle_note(action, &mut notes)
        }
    }
}  

fn load_notes_list() -> Vec<NoteID>{
    let mut notes = Vec::new();
    if let Some(dir) = PathBuf::from(".conf/.notes").parent() {
        if !dir.exists() {
            println!("No notes to load");
            return(notes);
        }
    }
    // Load notes from .conf/.notes
    let path = PathBuf::from(".conf/.notes");
    if let Ok(bytes) = fs::read(&path) {
        notes = bincode::deserialize(&bytes).unwrap_or_else(|_| Vec::new());
    }
    notes
}
fn roll(num: i32, die: i32) {//roll command ( rolls xdx ) USAGE: notus roll 2d8
    let mut rng = rand::thread_rng();
    println!("Rolling {}d{}:", num, die);
    let mut total = 0;
    for _ in 0..num {
        let roll: i32 = rng.gen_range(1..=die);
        println!("Rolled: {}", roll);
        total += roll;
    }
    println!("Total: {}", total);
}
/* Tests, they can be ignored (they like to throw warnings) 
Now that you are no longer paying attention, I can mention that the .notes in .conf is probably not the most efficinet datastorage
*/
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
       .stdout(predicates::str::contains(format!("Rolling {}d{}:", num, die)));
}