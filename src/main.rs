use std::{fs, path::PathBuf};

use cursive::traits::*;
use cursive::traits::With;
use cursive::theme::{ BorderStyle, Palette, PaletteColor::Highlight, PaletteColor::HighlightInactive};
use cursive::views::{ Button, Dialog, DummyView, LinearLayout, SelectView, TextView};
use cursive::Cursive;
use commands::*;
use notes:: NoteID;

mod cli;
mod commands;
mod notes;

fn main() {
    let notes_list = load_notes_list();
    
   
    let mut siv = cursive::default();
    /* 
     siv.set_theme(cursive::theme::Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette: Palette::retro().with(|palette| {
            use cursive::theme::BaseColor::*;
            {
                
                use cursive::theme::Color::TerminalDefault;
                use cursive::theme::PaletteColor::*;

                palette[Background] = TerminalDefault;
                palette[View] = TerminalDefault;
                palette[Primary] = White.dark();
                palette[TitlePrimary] = Red.light();
                palette[Secondary] = Red.light();
                palette[Highlight] = Red.dark();

            }
            {

                palette[Highlight] = Red.light();
                palette[HighlightInactive] = Yellow.dark();
            }
        }),
    });
*/
    siv.set_user_data(notes_list.clone());
    let notelist = SelectView::<String>::new().on_submit(|s, item| select_note(s, item)).with_name("notes").min_size((20, 5)).scrollable();
    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(notelist)
                .child(DummyView)
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

    siv.add_global_callback(cursive::event::Key::Esc, |s| { s.pop_layer(); });
    siv.add_global_callback('q', |s| { s.quit(); });

    let notes_clone = notes_list.clone();
    let cb_sink = siv.cb_sink().clone();
    siv.with_user_data(move |_notes: &mut Vec<NoteID>| {
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

// Displays options for the selected note
fn select_note(s: &mut Cursive, item: &str) {
    let item_clone_edit = item.to_string();
    let item_clone_delete = item.to_string();
    let item_clone_view = item.to_string();  
    let dialog = Dialog::text(item)
       
        .button("Edit", move |s| commands::edit_note(s, &item_clone_edit))
        .button("Delete", move |s| commands::delete_note(s, &item_clone_delete))
        .button("View", move |s| commands::view_note(s, &item_clone_view);
    s.add_layer(dialog);
}
fn load_notes_list() -> Vec<NoteID>{
    let mut notes = Vec::new();
    if let Some(dir) = PathBuf::from(".conf/.notes").parent() {
        if !dir.exists() {
            println!("No notes to load");
            return notes;
        }
    }
    // Load notes from .conf/.notes
    let path = PathBuf::from(".conf/.notes");
    if let Ok(bytes) = fs::read(&path) {
        notes = bincode::deserialize(&bytes).unwrap_or_else(|_| Vec::new());
    }
    notes
}

/* Tests, they can be ignored (they like to throw warnings) 
Now that you are no longer paying attention, I can mention that the .notes in .conf is probably not the most bestest storage system :0
*/
