
use crate::{cli::NoteAction, notes::{Note, NoteID}};
use std::fs;
use termimad::{crossterm::style::Print, MadSkin}; 
use serde_json;
use rand::Rng;
use scan_fmt::scan_fmt;
use std::path::PathBuf;
use crate::notes::{NoteType, save_notes_list};
use crate::notes;
pub fn handle_quicknote(input: &str, active_note: &NoteID) {
    let path = active_note.get_path().clone();
    let data = fs::read_to_string(&path).expect("Unable to read file");
    let mut note: Note = serde_json::from_str(&data).expect("Error parsing JSON");
    note.content += input;
    note.save();
    println!("Note updated and saved successfully.");
}
pub fn handle_roll(input: &str) {
    let (num, die) = scan_fmt!(input, "{}d{}", i32, i32).unwrap();
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
pub fn handle_note(action: NoteAction, notes: &mut Vec<NoteID>) {
    match action {
        NoteAction::New { name, tags } => {
            let note = Note::new(name.clone(), PathBuf::from(name), NoteType::Note, tags.split(',').map(String::from).collect(), notes);
            note.save();
            println!("{:?}", notes);
            save_notes_list(&notes);
        },
        NoteAction::View { name }=> {
            let skin = MadSkin::default();
            let index = &notes.iter().position(|r| r.name.to_lowercase() == name.to_lowercase()).unwrap();//finds the note to read
            // println!("{:?}", notes[index]);
            let note = fs::read_to_string(&notes[*index].path).unwrap();
            let deserialized: Note = serde_json::from_str(&note).unwrap();
            skin.print_text(&deserialized.content);
            
        },
        NoteAction::Active { name }=> {
            notes::set_active_note(notes, &name);
            println!("Active note set to {}", name);
            save_notes_list(&notes);
        },
        NoteAction::Edit => {
            save_notes_list(&notes);
         },
        NoteAction::Reset => {
            let notes_path = PathBuf::from("notes");
            println!("Current directory: {:?}", std::env::current_dir().unwrap());
            if notes_path.exists() && notes_path.is_dir() {
                let entries = std::fs::read_dir(&notes_path).expect("Failed to read contents");
                for entry in entries {
                    let entry = entry.expect("Failed to read entry");
                    let path = entry.path();
                    if path.is_file() { 
                        std::fs::remove_file(&path).expect("Failed to delete a file");
                    }
                }
            } else {
                println!("Notes directory does not exist or is not a directory.");
            }
            

            let conf_notes_path = PathBuf::from(".conf/.notes");
            println!("Absolute path for .conf/.notes file: {:?}", std::fs::canonicalize(&conf_notes_path));
            if conf_notes_path.exists() && conf_notes_path.is_file() {
                if let Err(e) = std::fs::remove_file(&conf_notes_path) {
                    println!("Failed to delete .conf/.notes file: {}", e);
                }
            } else {
                println!("ahaha .conf/.notes does not exist or is not a file.");
            }
            return;
        },
    }
    
}
