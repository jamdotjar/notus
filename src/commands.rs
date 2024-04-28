
use crate::{cli::NoteAction, notes::{Note, NoteID}};
use std::fs;

use serde_json;
use rand::Rng;
use scan_fmt::scan_fmt;
use std::path::PathBuf;
use crate::notes::{NoteType, save_notes_list};
use crate::notes;
use termimad::{MadSkin, Area, MadView, Error};
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyEvent, KeyCode::*},
    queue,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    style::Color::*,
};
use std::io::{stdout, Write};
use crossterm::style::Color::AnsiValue;

pub fn handle_quicknote(input: &str, active_note: &NoteID) {
    let path = active_note.get_path().clone();
    let data = fs::read_to_string(&path).expect("Unable to read file");
    let mut note: Note = serde_json::from_str(&data).expect("Error parsing JSON");
    note.content += input;
    note.content += "\n";
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
            let skin = make_skin();
            let index = &notes.iter().position(|r| r.name.to_lowercase() == name.to_lowercase()).unwrap();//finds the note to read
            let note = fs::read_to_string(&notes[*index].path).unwrap();
            let deserialized: Note = serde_json::from_str(&note).unwrap();
            display(skin, &deserialized.content);
            
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

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.table.align = termimad::Alignment::Center;
    skin.set_headers_fg(AnsiValue(178));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(Magenta);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin.code_block.align = termimad::Alignment::Center;
    skin
}
fn view_area() -> Area {
    let mut area = Area::full_screen();
    area.pad_for_max_width(120); // Limit width to 120 for better readability
    area
}
fn display(skin: MadSkin, markdown: &str) -> Result<(), Error> {
    let mut w = stdout();
    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, Hide)?; // Hide the cursor

    let mut view = MadView::from(markdown.to_owned(), view_area(), skin);

    loop {
        view.write_on(&mut w)?;
        w.flush()?;
        match event::read() {
            Ok(Event::Key(KeyEvent { code, .. })) => match code {
                Up => view.try_scroll_lines(-1),
                Down => view.try_scroll_lines(1),
                PageUp => view.try_scroll_pages(-1),
                PageDown => view.try_scroll_pages(1),
                _ => break,
            },
            Ok(Event::Resize(..)) => {
                queue!(w, Clear(ClearType::All))?;
                view.resize(&view_area());
            }
            _ => {}
        }
    }

    terminal::disable_raw_mode()?;
    queue!(w, Show)?; // Restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}
