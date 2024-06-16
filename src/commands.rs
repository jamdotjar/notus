use crate::notes::{save_notes_list, Note, NoteID, NoteType};
use cursive::theme::{BaseColor, Color, PaletteColor, Theme};
use cursive::{traits::*, views::*, Cursive};
use rand::Rng;
use serde_json::to_string_pretty;
use std::fmt::write;
use std::fs;
use std::path::PathBuf;
/* quicknote
pub fn handle_quicknote(input: &str, active_note: &NoteID) {
    let path = active_note.get_path().clone();
    let data = fs::read_to_string(&path).expect("Unable to read file");
    let mut note: Note = serde_json::from_str(&data).expect("Error parsing JSON");
    note.content += input;
    note.content += "\n";
    note.save();
    println!("Note updated and saved successfully.");
} */

#[allow(dead_code)]
// roll command ( rolls xdx ) USAGE: notus roll 2d8
fn roll(num: i32, die: i32) {
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


// Deletes note with name "item"
pub fn delete_note(s: &mut Cursive, item: &str) {
    s.call_on_name("notes", |view: &mut SelectView<String>| {
        let index = view.iter().position(|(label, _)| label == item);
        if let Some(index) = index {
            view.remove_item(index);
        }
    });

    if let Some(index) = s
        .with_user_data(|note_list: &mut Vec<NoteID>| note_list.iter().position(|n| n.name == item))
    {
        s.with_user_data(|note_list: &mut Vec<NoteID>| {
            let note = note_list.remove(index.unwrap());
            if let Err(e) = std::fs::remove_file(&note.path) {
                eprintln!("Failed to delete note file: {}", e);
            }
            save_notes_list(note_list);
        });
    }

    s.pop_layer();
}
pub fn view_note(s: &mut Cursive, item: &str){
    let note = s.with_user_data(|note_list: &mut Vec<NoteID>| {
        note_list.iter().find(|n| n.name == item).cloned()
    });

    
    
}
pub fn edit_note(s: &mut Cursive, item: &str) {
    let note = s.with_user_data(|note_list: &mut Vec<NoteID>| {
        note_list.iter().find(|n| n.name == item).cloned()
    });

    if let Some(note) = note {
        let note = note.unwrap();
        let path = note.get_path().clone();
        let raw_note = std::fs::read_to_string(&path).expect("Failed to read file");
        let mut note_content: Note =
            serde_json::from_str(&raw_note).expect("Failed to deserialize content");
        let content_of_content = note_content.content.replace("\\n", "\n");

        s.add_layer(
            Dialog::new()
                .content(ThemedView::new(
                    editor_theme(),
                    TextArea::new()
                        .content(content_of_content)
                        .with_name("edit_content")
                        .full_screen(),
                ))
                .button("Save", move |s| {
                    let text = s
                        .call_on_name("edit_content", |view: &mut TextArea| {
                            view.get_content().to_string()
                        })
                        .unwrap();
                    let mut note_content: Note =
                        serde_json::from_str(raw_note.as_str()).expect("Failed to read note");
                    note_content.content = text.clone();
                    let serialized = serde_json::to_string_pretty(&note_content)
                        .expect("Content didnt serialize");

                    fs::write(&path, serialized);
                    fs::write("console.txt", text.clone()).expect("Failed to write to file");

                    s.pop_layer();
                    s.add_layer(TextView::new(text));
                })
                .button("Cancel", |s| {
                    s.pop_layer();
                }),
        );
    } else {
        s.add_layer(Dialog::info("Note file not found."));
    }
}
pub fn create_note_screen(s: &mut Cursive) {
    s.add_layer(
        Dialog::new()
            .title("Enter a new name")
            .content(EditView::new().with_name("name").fixed_width(10))
            .button("Ok", |s| {
                let name = s
                    .call_on_name("name", |v: &mut EditView| v.get_content())
                    .unwrap();
                s.with_user_data(|notes: &mut Vec<NoteID>| {
                    let note = Note::new(
                        name.to_string(),
                        PathBuf::from(&*name),
                        NoteType::Note,
                        Vec::new(),
                        notes,
                    );
                    note.save();
                    save_notes_list(notes);
                });
                s.call_on_name("notes", |view: &mut SelectView<String>| {
                    view.add_item_str(&*name);
                });
                s.pop_layer();
            })
            .button("Cancel", |s| {
                s.pop_layer();
            }),
    );
}
fn editor_theme() -> Theme {
    let theme = Theme::default();
    theme
}
