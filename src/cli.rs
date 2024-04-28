use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "notus", about="Notes for us", long_about = "A DND notes app with insane functionality (dungeon generation, dice rolling, markdown support, exporting)")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands, 

    #[arg( required = false)]    
    pub note: Option<String>,
    
}
#[derive(Subcommand)]
pub enum Commands {
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
pub enum NoteAction {//handles differnet flags for different functions
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

