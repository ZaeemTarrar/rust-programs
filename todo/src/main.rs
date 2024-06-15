use std::process;

mod library;
use crate::library::todos::todos::Note;
use crate::library::menus::menus::{ Menu, Msg, enter_to_continue };
use crate::library::utils::utils::console_clear;

fn main() {
    let mut notes: Vec<Note> = Vec::new();
    notes.push(Note::new(String::from("Zaeem"), String::from("Hello World"), false));
    notes.push(Note::new(String::from("Noor"), String::from("Hello There"), false));
    notes.push(Note::new(String::from("Shahab"), String::from("Wow, Great !"), true));
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
                for (i, note) in notes.iter_mut().enumerate() {
                    println!("{}> {}", i + 1, note.info());
                    note.set_read(true);
                }
                enter_to_continue();
            }
            3 => {
                Msg::clear_alert("Delete a Note");
                let del_id: u8 = Menu::del_note();
                if del_id != 0 && (del_id as usize) < notes.len() {
                    notes.remove((del_id as usize) - 1);
                    Msg::flash(&format!("Note:{} deleted successfully", del_id));
                } else {
                    Msg::flash("Note id doesn't Exist");
                }
            }
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
