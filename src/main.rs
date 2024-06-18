mod commands;
mod notes;
use notes::export;
use std::{fs, path::PathBuf};
use cursive::traits::*;
use commands::create_note;
use cursive::views::{ Button, Dialog, LinearLayout, SelectView, TextView};
use cursive::Cursive;
use cursive::theme::{BorderStyle, Palette, PaletteColor, Color, BaseColor, Theme};
use notes::NoteID;

fn main() {
     //mandatory declaration of important stuff
    let notes_list = load_notes_list();   
    let mut siv = cursive::default();
   
    let mut palette = Palette::terminal_default();
    palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::Primary] = Color::Light(BaseColor::White);

    siv.set_theme(Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette,
    });
    siv.set_user_data(notes_list.clone());
    let notelist = SelectView::<String>::new().on_submit(|s, item| select_note(s, item)).with_name("notes").min_size((20, 5)).scrollable();
    siv.add_layer(
        Dialog::around(//Homepage
            LinearLayout::horizontal()
                .child(
                    notelist.full_width()// List of notes
                ) .child(TextView::new("│││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││││").full_height().fixed_width(1))// this sucks but im not changing it
                .child(
                    LinearLayout::vertical()//Sidebar options
                        .child(TextView::new("Options:"))
                        .child(Button::new("New note", move |s: &mut Cursive| {
                            create_note(s)
                        })).child(Button::new("Quit", |s| s.quit()))
                )
        )
        .title("NotUS")
        .min_size((60, 15))
        .max_size((80, 25))

    );

    //Binds ESC to closing current window ( or quitting app )
    siv.add_global_callback(cursive::event::Key::Esc, |s| {
        if s.screen().len() <= 1 {
            s.quit();
        } else {
            s.pop_layer();
        }
    });
    let notes_clone = notes_list.clone();
    let notes_clone_for_view = notes_clone.clone();
    siv.with_user_data(move |notes: &mut Vec<NoteID>| {//this is how Cursive handles globals, kinda
        let notes = notes_clone.clone();
    });
    siv.call_on_name("notes", move |view: &mut SelectView<String>| {//populating the display from the notes list
        for note in notes_clone_for_view.iter() {
            view.add_item_str(&note.name);
        }
    });
    siv.run();
}  

// Displays options for the selected note
fn select_note(s: &mut Cursive, item: &str) {
    let item_clone_delete = item.to_string();
    let item_clone_view = item.to_string();  
    let dialog = Dialog::text(item)
       
        .button("Open", move |s| {s.pop_layer(); commands::view_note(s, &item_clone_view)})
        .button("Delete", move |s| commands::delete_note(s, &item_clone_delete))
        .button("Export", move |s| notes::export());

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
