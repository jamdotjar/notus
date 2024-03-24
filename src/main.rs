use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use clap::{Parser, Subcommand};
use std::process::Command; // Run programs
use std::path::PathBuf;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::{predicate::str, *}; // Used for writing assertions
use scan_fmt::scan_fmt;
use chrono::DateTime;
use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Parser)]
#[command(name = "notus", about="Notes for us", long_about = "A DND notes app with insane functionality (dungeon generation, dice rolling, markdown support, exporting)")]
struct Cli {
    #[arg( required = false)]    
    note: Option<String>,

   
    #[clap(subcommand)]
    command: Commands,
    

}
#[derive(Subcommand)]
enum Commands {
    /// Rolls a die
    Roll {
        #[arg()]
        input: String,
    },
    /// Note global comand
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
   ID: NoteID,
   content: String,
}
impl Note {
    fn new(mut name: String,mut path: PathBuf, note_type: NoteType, tags: Vec<String>, notes: &mut Vec<NoteID>) -> Self {
        let now = Utc::now().to_string();
        let content = String::new();
        // Check if a file with the given path already exists
        let mut counter = 1;
        let original_file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();
        let oldname = name.clone();
        while path.exists() {
            
            let new_path = path.with_file_name(format!("{}{}.json", original_file_stem, counter));
            path = new_path;
            name = oldname.clone()+&counter.to_string();
            counter += 1;
        }
        let id = Note::generate_note_id(&name, &path);
        notes.push(id.clone());
        Self {
            name,
            path,
            note_type,
            tags,
            date: now,
            ID: id,
            content: content,
        }
     
    }
    fn save(&self){
      // Note { date: 2024-03-17T00:13:41.073965Z, name: "Hello", path: "Hello", note_type: Note, tags: [], ID: Id(258311098) }
      
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
    fn generate_note_id(name: &String, path: &PathBuf) -> NoteID {
        let mut hasher = DefaultHasher::new();
        path.hash(&mut hasher);
        let hash = hasher.finish();
        let id: NoteID = NoteID::Id(hash as u32);
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
enum NoteID {
    Name(String),
    Id(u32),
    Path(PathBuf)
}

fn main() {
//initalize cli and Random number generator
    let mut notes: Vec<NoteID> = vec![];
    let cli = Cli::parse();
    let mut rng = rand::thread_rng();
    if let Some(note) = cli.note {
        println!("{}", note);
    }
    else {
       match &cli.command {
        Commands::Roll { input } => {
            let (num, die) = scan_fmt!(
                input, "{}d{}", i32, i32).unwrap();
            roll(num, die, &mut rng)
        }
        Commands::Note { new, active, edit, name, tags} => {
            let path = PathBuf::from(format!("./notes/{}.json", name));
            let tags = tags.split(',').map(|s| s.trim().to_string()).collect();
            if *new == true {
                let note = Note::new(name.clone(),
                path, NoteType::Note,
                tags, &mut notes);
                note.save();
            }
        }   
        _ =>{ println!("you need to write something man") }
    }
   
}  
}

fn roll(num: i32, die: i32, rng: &mut rand::rngs::ThreadRng){
    println!("Rolling {}d{}:", num, die);
    let mut roll = 0;
    for i in 0..num{
        let rolled: i32 = rng.gen_range(1..=die);
        println!("Roll {}: {}",i+1, rolled);
        roll+= rolled;
    }
    println!("Total: {}", roll);
}

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
       .stdout(predicate::str::contains(format!("Rolling {}d{}:", num, die)));
}