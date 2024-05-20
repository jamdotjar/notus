use std::{path::PathBuf, thread::AccessError};
use std::fs;

use rand::Rng;
use std::hash::{Hash, DefaultHasher, Hasher};
use std::process::Command; // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use scan_fmt::scan_fmt;
use chrono::Utc;
use crate::notes::Note;
use cursive::views::{BoxedView, Dialog, Button, LinearLayout, TextView, SelectView, ResizedView};
use cursive::traits::*;
use cursive::Cursive;
mod notes;
mod cli;
mod commands;
use cursive::immut1;

use cli::{Cli, Commands, NoteAction};
use notes::{save_notes_list, NoteID};
use commands::{create_note_screen, delete_note, select_note};

fn main() {
    let mut notes = load_notes_list();
    let mut active: Option<NoteID> = notes.iter().find(|n| n.active).cloned();
    
    let mut siv = cursive::default();
    siv.set_user_data(notes.clone());
    let mut notelist = SelectView::<String>::new().on_submit(|s, item| select_note(s, item)).with_name("notes");
    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(ResizedView::with_min_size((20, 5), notelist))
                .child(
                    LinearLayout::vertical()
                        .child(TextView::new("Options:"))
                        .child(Button::new("New note", move |s: &mut Cursive| {
                            create_note_screen(s)
                        })).child(Button::new("Quit", |s| s.quit()))
                )
        )
        .title("NotUS")
    );

    let notes_clone = notes.clone();
    let cb_sink = siv.cb_sink().clone();
    siv.with_user_data(move |notes: &mut Vec<NoteID>| {
        let notes = notes_clone.clone();
        cb_sink.send(Box::new(move |s| {
            s.call_on_name("notes", |view: &mut SelectView<String>| {
                for note in notes.iter() {
                    view.add_item_str(&note.name);
                }
            });
        })).unwrap();
    });
    siv.run();

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
