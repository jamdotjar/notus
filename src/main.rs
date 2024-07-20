mod commands;
mod notes;
use commands::{create_note, import_note};
use cursive::theme::{BaseColor, BorderStyle, Color, Palette, PaletteColor, Theme};
use cursive::traits::*;
use cursive::views::{Button, Dialog, LinearLayout, SelectView, TextView};
use cursive::Cursive;
use notes::load_notes_list;

fn main() {
    //mandatory declaration of important vars
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
    let notelist = SelectView::<String>::new()
        .on_submit(select_note)
        .with_name("notes")
        .min_size((20, 5))
        .scrollable();
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
                        })).child(Button::new("Import", |s: &mut Cursive| { import_note(s)})).child(Button::new("Quit", |s| s.quit()))
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

    let notes_clone_for_view = notes_list.clone();
    siv.call_on_name("notes", move |view: &mut SelectView<String>| {
        //populating the display from the notes list
        for note in notes_clone_for_view.iter() {
            view.add_item_str(&note.name);
        }
    });
    siv.run();
}

// Displays options for selected note
fn select_note(s: &mut Cursive, item: &str) {
    let item_clone_delete = item.to_string();
    let item_clone_view = item.to_string();
    let item_clone_export = item.to_string(); // Add this line
    let dialog = Dialog::text(item)
        .button("Open", move |s| {
            s.pop_layer();
            commands::view_note(s, &item_clone_view)
        })
        .button("Delete", move |s| {
            commands::delete_note(s, &item_clone_delete)
        })
        .button("Export", move |s| {
            commands::export_note(s, &item_clone_export)
        }); // Update this line

    s.add_layer(dialog);
}
