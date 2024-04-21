use std::{path::PathBuf, thread::AccessError};
use std::fs;
use rand::Rng;
use std::hash::{Hash, DefaultHasher, Hasher};
use clap::{Parser, Subcommand};
use std::process::Command; // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use scan_fmt::scan_fmt;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use termimad::MadSkin; //I am fully convinced this is a send from the skies

#[derive(Parser)]
#[command(name = "notus", about="Notes for us", long_about = "A DND notes app with insane functionality (dungeon generation, dice rolling, markdown support, exporting)")]
struct Cli {
    #[command(subcommand)]
    command: Commands, 

    #[arg( required = false)]    
    note: Option<String>,
    
}
#[derive(Subcommand)]
enum Commands {
    /// Rolls a die
    Roll {
        #[arg()]
        input: String,
    },

    Note {//general note command
        #[clap(subcommand)]
        action: NoteAction, //passes to flag handler
    },

   #[clap(short_flag='t', long_flag= "quick")]
    Quick {
        #[arg()]
        input: String,
    }

}
#[derive(Subcommand)]
enum NoteAction {//handles differnet flags for different functions
    #[clap(name="New", short_flag = 'n', long_flag = "new", about="Create a new note")]
    New {
        name: String,
        #[clap(short = 't', long = "tags")]
        tags: String,
    },
    #[clap(name="View", short_flag = 'v', long_flag = "view")]
    View {
        name: String,
    },
    #[clap(name="Active", short_flag = 'a', long_flag = "active")]
    Active {
        name: String,
    },
    Edit,
    #[clap(name="Reset", short_flag= 'r', long_flag = "reset")]
    Reset, 
}


#[derive(Serialize, Deserialize,  Debug)]
struct Note {
   date: String,
   name: String,
   path: PathBuf,
   note_type: NoteType,
   tags: Vec<String>, 
   id : NoteID,
   content: String,
}
impl Note {
   fn new(mut name: String, mut path: PathBuf, note_type: NoteType, tags: Vec<String>, notes: &mut Vec<NoteID>) -> Self {
    let now = Utc::now().to_string();
    let content = String::new();
    // Ensure the base directory is 'notes/'
    path = PathBuf::from("notes").join(&path);
    path.set_extension("json");  // Set the extension before the loop
    let original_file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();
    let mut counter = 0;
    while path.exists() {
        counter += 1;
        let new_file_name = format!("{}{}.json", original_file_stem, counter);

        path = PathBuf::from("notes").join(&new_file_name);

    }
    if counter > 0 {
        name = format!("{}{}", original_file_stem, counter);
    }
    let id = Note::generate_note_id(&name, &path);
    notes.push(id.clone());
    set_active_note(notes, &id.name);
    Self {
        name,
        path,
        note_type,
        tags,
        date: now,
        id: id,
        content: content,
    }
}
    fn save(&self){
      // Note { date: 2024-03-17T00:13:41.073965Z, name: "Hello", path: "Hello", note_type: Note, tags: [], id: Id(258311098) }
        // make dir if not there
        if let Some(dir) = self.path.parent() {
            if !dir.exists() {
                std::fs::create_dir_all(&dir).unwrap();
            }
        }
        let serialized = serde_json::to_string_pretty(&self).unwrap();
        //try and write
        if let Err(e) = std::fs::write(&self.path, &serialized) {
            eprintln!("Failed to write to file: {}", e);
        }
        println!("Created: {}", &self.name)
    }
    fn generate_note_id(name: &String, path: &PathBuf) -> NoteID {//generates a NoteID object with a hash (from path, which is from name, i love this system)
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        path.hash(&mut hasher);
        let hash = hasher.finish();
        let id = NoteID { name: name.clone(), id: hash as u32, path: path.clone(), active: true};
        id
    }
    
      

}
#[derive(Debug, Serialize, Deserialize)]
enum NoteType {
    Note,
    Sheet,
    Character
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct NoteID {  //noteID for notes array
    name: String,
    id: u32,
    path: PathBuf,
    active: bool,
}

fn set_active_note(notes: &mut Vec<NoteID>, active_note_name: &str) {
    for note in notes.iter_mut() {
        note.active = note.name == active_note_name;
    }
}

fn main() {
//initalize cli and Random number generator
    let cli = Cli::parse();
    let mut notes = load_notes_list();
    let mut active: Option<NoteID> = notes.iter().find(|n| n.active).cloned();

    //println!("{:?}", notes);
    match &cli.command {
        Commands::Roll { input } => {
            let (num, die) = scan_fmt!(input, "{}d{}", i32, i32).unwrap();
            roll(num, die);
        },
        Commands::Quick { input} => {
            let path = active.unwrap().path.clone();
            let data = fs::read_to_string(&path).expect("Unable to read file");
            let mut note: Note = serde_json::from_str(&data).expect("Error parsing JSON");
            note.content += input;
            note.save();
            println!("Note updated and saved successfully.");
        },
        Commands::Note { action } => {
            match action {
                NoteAction::New { name, tags } => {
                    let note = Note::new(name.clone(), PathBuf::from(name), NoteType::Note, tags.split(',').map(String::from).collect(), &mut notes);
                    note.save();
                    println!("{:?}", notes);
                },
                NoteAction::View { name }=> {
                    let mut skin = MadSkin::default();
                    let index = &notes.iter().position(|r| r.name.to_lowercase() == name.to_lowercase()).unwrap();//finds the note to read
                    // println!("{:?}", notes[index]);
                    let note = fs::read_to_string(&notes[*index].path).unwrap();
                    let deserialized: Note = serde_json::from_str(&note).unwrap();
                    skin.print_text(&deserialized.content);
                },
                NoteAction::Active { name }=> {
                    set_active_note(&mut notes, name);
                    println!("Active note set to {}", name)
                },
                NoteAction::Edit => {
//
                 },
                NoteAction::Reset => {
                    let notes_path = PathBuf::from("notes");
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
                    println!("Absolute path for .conf/.notes file: {:?}", std::fs::canonicalize(&conf_notes_path).unwrap());
                    if conf_notes_path.exists() && conf_notes_path.is_file() {
                        std::fs::remove_file(&conf_notes_path);

                    } else {
                        println!("The .conf/.notes does not exist or is not a file.");
                    }
                    return;
                },
            }
        }
    }
   save_notes_list(&notes);
}  

fn save_notes_list(notes: &Vec<NoteID>){
    let encoded: Vec<u8> = bincode::serialize(notes).unwrap();
    if let Some(dir) = PathBuf::from(".conf/.notes").parent() { //create drectory if not present
        if !dir.exists() { 
            std::fs::create_dir_all(dir).unwrap();
        }
    }
    fs::write(".conf/.notes", &encoded).expect("Couldnt save note data");
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