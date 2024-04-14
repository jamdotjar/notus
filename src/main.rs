use std::path::PathBuf;
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


#[derive(Parser)]
#[command(name = "notus", about="Notes for us", long_about = "A DND notes app with insane functionality (dungeon generation, dice rolling, markdown support, exporting)")]
struct Cli {
    #[arg( required = false)]    
    note: Option<String>,

    #[command(subcommand)]
    command: Commands, 

}

//
#[derive(Subcommand)]
enum Commands {
    /// Rolls a die
    Roll {
        #[arg()]
        input: String,
    },

    /// Note global comands
    Note {
        #[clap(short = 'n', long = "new")]
        new: bool,
        #[clap(short = 'a', long = "active")]
        active: bool,
        #[clap(short = 'e', long = "edit")]
        edit: bool,
        #[arg()]
        name: String,
        #[clap(short = 't', long = "tags")]
        tags: String,
    }

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

        // REMEMBER THIS IF I EVER HAVE TO REWRITE
        path = PathBuf::from("notes").join(path);

        let original_file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();
        let mut counter = 0;
        while path.exists() {
            counter += 1;
            let new_file_name = format!("{}{}", original_file_stem, counter);
            path.set_file_name(new_file_name);        // some stupid stuff to make sure all the files have a .json
        }
        if counter > 0 {
            name = format!("{}{}", original_file_stem, counter);
        }
        path.set_extension("json");//this is kinda maybe sorta important if i dont wanna go cray cray
        let id = Note::generate_note_id(&name, &path);
        notes.push(id.clone());
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
        print!("{:?}", self.path);
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
        print!("Created: {}", &self.name)
    }
    fn generate_note_id(name: &String, path: &PathBuf) -> NoteID {//generates a NoteID object with a hash (from path, which is from name, i love this system)
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        path.hash(&mut hasher);
        let hash = hasher.finish();
        let id = NoteID { name: name.clone(), id: hash as u32, path: path.clone() };
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
}


fn main() {
//initalize cli and Random number generator
    let cli = Cli::parse();
    let mut notes = load_notes_list();
    println!("{:?}", notes);
    if let Some(note) = cli.note {
        println!("{}", note);
    }
    else {
       match &cli.command {
        Commands::Roll { input } => {
            let (num, die) = scan_fmt!(
                input, "{}d{}", i32, i32).unwrap();
            roll(num, die)
        },//example: notus roll 5d8
        Commands::Note { new, active, edit, name, tags } => {
            // Handle the Note command here
            // For example, create a new note if the 'new' flag is true
            if *new {
                let note = Note::new(name.clone(), PathBuf::from(name), NoteType::Note, tags.split(',').map(String::from).collect(), &mut notes );
                note.save();
            }
        },
        _ =>{ println!("you need to write something man") }
    }  
    }
   save_notes_list(&notes)
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