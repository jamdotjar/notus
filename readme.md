# Notus: Notes for Us (A DnD notes app built in Rust 🦀)
[![crates.io](https://img.shields.io/crates/v/notus?logo=rust&logoColor=white&style=flat-square)](https://crates.io/crates/notus)

Notus is a command line application for markdown note-taking. It is still in heavy development and is essentially my way of learning Rust so expect some slightly mediocre code.

## Installation
notus can be installed using cargo, the Rust package manager. To install notus, run the following command in your terminal:

```bash
cargo install notus
```

## Usage
### Launching the Application
To launch the application, run the following command in your terminal:

```bash
notus
```
![image](https://github.com/user-attachments/assets/a78d207b-ff00-4fe2-847a-6c2a5934bafd)

### Creating a Note
1. Launch the application.
2. Click on the "New note" button.
3. Enter the name of the note and click "Ok".
4. Edit the note content and click "Save".

![image](https://github.com/user-attachments/assets/0329c410-bb6c-4b67-8758-9ad512cb58f6)

### Viewing a Note
1. Select a note from the list by pressing enter ( or clicking if your terminal is fancy )
2. Click "Open" to view the note.

### Deleting a Note
1. Select a note from the list.
2. Click "Delete" to remove the note.

### Exporting Notes
1. Select the note you want to export.
2. Click on the "Export" button.
3. Choose the desired format (e.g., markdown).
4. Click "Save" to export the note to your downloads.
### Importing Notes
1. Click on the "Import" button.
2. Choose the file you want to import.
3. Click "Open" to import the file.



## Features

## Features Checklist

- [ ] **Dice Rolling**: Roll dice... groundbreaking, I know.
- [x] **Note Management**: Create, edit, and otherwise manage notes 📝.
- [x] Create
- [x] View
- [x] Management
-  [x] Delete
-  [ ] Rename
- [x] edit


- [ ] **Dungeon Generation**: Generate dungeons and export them for use in VTT 🏰.

- [ ] **Character Sheet Support**: Create and import character sheets 📚.
- [x] **Import**: import markdown files. (NOTE: currently must use a filepicker to import files, I am looking into a full TUI file picker)
- [x] **Export**: Export your notes in various formats, roll skill checks, and track spells, inventory, etc. 📦.
    - [x] markdown export.

- [ ] **Docs**: D&D is complicated; use the inbuilt docs to quickly look up spells, monster stats, loot tables, magic items, and more! 📖.


## License

Notus is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. For more information, see `LICENSE.txt`.
