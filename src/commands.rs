use crate::notes::{save_notes_list, Note, NoteID, NoteType};
use cursive::{traits::*, views::*, Cursive};
use cursive::utils::markup::markdown;
use rand::Rng;
use cursive::theme::{BaseColor, Color, PaletteColor, Theme};
use std::fs;
use std::path::PathBuf;
use cursive::views::ThemedView;

#[allow(dead_code)]
// TODO: use this somewhere
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
    if let Some(note) = note {
        let path = note.unwrap().get_path().clone();
        let raw_note = std::fs::read_to_string(&path).expect("Failed to read file");
        let note_content = serde_json::from_str::<Note>(&raw_note)
            .expect("Failed to deserialize content")
            .content
            .replace("\\n", "\n");

        s.add_layer(Dialog::new()
.content(
            ScrollView::new(TextView::new(markdown::parse(note_content)))
        ).button("Edit", {
            let item = item.to_string();
            move |s| {
                s.pop_layer();
                
                edit_note(s, &item);
            }
        }).button( "Close", move |s| {
            s.pop_layer();
        }
        ).fixed_size((70, 20)));
    }
    
    
}
pub fn edit_note(s: &mut Cursive, item: &str) {
    let note = s.with_user_data(|note_list: &mut Vec<NoteID>| {
        note_list.iter().find(|n| n.name == item).cloned()
    });

    if let Some(note) = note {
        let path = note.unwrap().get_path().clone();
        let raw_note = std::fs::read_to_string(&path).expect("Failed to read file");
        let note_content = serde_json::from_str::<Note>(&raw_note)
            .expect("Failed to deserialize content")
            .content
            .replace("\\n", "\n");
        let item_01 = item.to_string();
        let item_02 = item.to_string();
        let custom_theme = cursive::theme::Theme::retro();
        s.add_layer(
            Dialog::new()
            .content(
                ThemedView::new(
                    custom_theme,
                    TextArea::new()
                        .content(note_content)
                        .with_name("edit_content")
                )
            )
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
                       
                   
                    fs::write(&path, serialized).expect("Could not save note");
                    fs::write("console.txt", text.clone()).expect("Failed to write to file");
                    s.pop_layer();
                    view_note(s, &item_01);
                })
                .button("Cancel", move |s| {
                    s.pop_layer();
                    view_note(s, &item_02);
                }).fixed_size((70, 22)),
        );

    } else {
        s.add_layer(Dialog::info("Note file not found."));
    }
}
pub fn create_note(s: &mut Cursive) {
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
                edit_note(s, &name)
            })
            .button("Cancel", |s| {
                s.pop_layer();
            }),
    );
}
