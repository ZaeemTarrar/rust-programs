use std::process;

mod library;
use crate::library::todos::todos::Note;
use crate::library::menus::menus::{ Menu, Msg, enter_to_continue };
use crate::library::utils::utils::console_clear;

fn main() {
    let mut notes: Vec<Note> = Vec::new();
    Menu::welcome();
    loop {
        match Menu::main_menu() {
            1 => {
                let note: Note;
                loop {
                    Msg::clear_alert("New Note");
                    let (title, desc) = Menu::new_note();
                    if title.is_empty() || desc.is_empty() {
                        Msg::flash("Invalid Input. Try Again !");
                    } else {
                        note = Note::new(title, desc, false);
                        break;
                    }
                }
                notes.push(note);
                Msg::flash("New Note Added Successfully");
            }
            2 => {
                Msg::clear_alert("List of all Notes");
                for (i, note) in notes.iter().enumerate() {
                    println!("{}> {}", i + 1, note.info());
                }
                enter_to_continue();
            }
            3 => {}
            4 => {
                Msg::clear_alert("Exiting Program");
                process::exit(0);
            }
            _ => {
                Msg::flash("Invalid Option. Try Again!");
            }
        }
        console_clear();
    }
}
