use crate::notes::{export, save_notes_list, Note, NoteID, NoteType};
use cursive::utils::markup::markdown;
use cursive::views::ThemedView;
use cursive::{traits::*, views::*, Cursive};
use dirs::home_dir;
use rand::Rng;
use rfd::FileDialog;
use std::fs::{self, write};
use std::path::PathBuf;

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

pub fn import_note(s: &mut Cursive) {
    let file = match FileDialog::new()
        .add_filter("markdown", &["md", "txt"])
        .set_directory(home_dir().unwrap_or_else(|| PathBuf::from("/")))
        .pick_file()
    {
        Some(path) => path,
        None => return,
    };

    // TODO: fix crash when cancel is presed on dialouge
    let content = match fs::read_to_string(&file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };
    let name = file
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    s.with_user_data(|notes: &mut Vec<NoteID>| {
        let mut note = Note::new(
            name.clone(),
            PathBuf::from(name.clone()),
            NoteType::Note,
            Vec::new(),
            notes,
        );
        note.content = content;
        note.save();
        save_notes_list(notes);
    });
    s.call_on_name("notes", |view: &mut SelectView<String>| {
        view.add_item_str(name);
    });
}
pub fn export_note(s: &mut Cursive, item: &str) {
    let path = get_notepath(item, s);
    let content = {
        match std::fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<Note>(&content) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Failed to deserialize content: {}", e);
                    return;
                }
            },
            Err(e) => {
                eprintln!("Failed to read file {}", e);
                return;
            }
        }
    };
    let usrpath = export(&content.content, &content.name);
    s.add_layer(
        Dialog::new()
            .title("Export successful")
            .content(TextView::new(format!("notename is stored at {}", usrpath)))
            .button("Okay", |s| {
                s.pop_layer();
            }),
    );
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
            if let Err(e) = std::fs::remove_file(note.path) {
                eprintln!("Failed to delete note file: {}", e);
            }
            save_notes_list(note_list);
        });
    }
    s.pop_layer();
}
pub fn view_note(s: &mut Cursive, item: &str) {
    let path = get_notepath(item, s);
    let raw_note = std::fs::read_to_string(path).expect("Failed to read file");
    let note_content = match serde_json::from_str::<Note>(&raw_note) {
        Ok(content) => content.content.replace("\\n", "\n"),
        Err(e) => {
            eprintln!("Failed to deserialize content: {}", e.to_string());
            return;
        }
    };
    s.add_layer(
        Dialog::new()
            .content(ScrollView::new(TextView::new(markdown::parse(
                note_content,
            ))))
            .button("Edit", {
                let item = item.to_string();
                move |s| {
                    s.pop_layer();
                    edit_note(s, &item);
                }
            })
            .button("Close", move |s| {
                s.pop_layer();
            })
            .fixed_size((70, 20)),
    );
}

pub fn edit_note(s: &mut Cursive, item: &str) {
    let path = get_notepath(item, s);
    let raw_note = std::fs::read_to_string(&path).expect("Failed to read file");
    let note_content = match serde_json::from_str::<Note>(&raw_note) {
        Ok(content) => content.content.replace("\\n", "\n"),
        Err(e) => {
            eprintln!("Failed to deserialize content: {}", e.to_string());
            return;
        }
    };
    let item_01 = item.to_string();
    let item_02 = item.to_string();
    let custom_theme = cursive::theme::Theme::retro();
    s.add_layer(
        Dialog::new()
            .content(ThemedView::new(
                custom_theme,
                TextArea::new()
                    .content(note_content)
                    .with_name("edit_content"),
            ))
            .button("Save", move |s| {
                let text = s
                    .call_on_name("edit_content", |view: &mut TextArea| {
                        view.get_content().to_string()
                    })
                    .unwrap();
                let mut note_content: Note = match serde_json::from_str(raw_note.as_str()) {
                    Ok(note) => note,
                    Err(e) => {
                        eprintln!("Failed to deserialize content: {}", e.to_string());
                        return;
                    }
                };

                note_content.content = text.clone();

                let serialized = match serde_json::to_string_pretty(&note_content) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("Failed to serialize content: {}", e.to_string());
                        return;
                    }
                };

                match fs::write(&path, serialized) {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Failed to save note: {}", e.to_string());
                        return;
                    }
                };

                s.pop_layer();
                view_note(s, &item_01);
            })
            .button("Cancel", move |s| {
                s.pop_layer();
                view_note(s, &item_02);
            })
            .fixed_size((70, 22)),
    );
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

fn get_notepath(item: &str, s: &mut Cursive) -> PathBuf {
    s.with_user_data(|note_list: &mut Vec<NoteID>| {
        note_list.iter().find(|n| n.name == item).cloned()
    })
    .map(|note| note.unwrap().get_path().clone())
    .expect("Note not found")
}
