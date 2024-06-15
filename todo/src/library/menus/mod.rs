pub mod menus {
    use crate::library::utils::utils::{ console_clear, wait };
    use std::io::{ Write, stdin, stdout };

    pub fn take_input(title: &str) -> String {
        print!("{} ", title);
        stdout().flush().expect("Flush Error");
        let mut opt: String = String::new();
        stdin().read_line(&mut opt).expect("Input Read Error");
        return opt.trim().to_string();
    }

    pub fn enter_to_continue() {
        super::menus::take_input("\n=> [ Press Enter to Go Back ]...");
        wait(1);
        console_clear();
    }

    pub struct Menu {}
    impl Menu {
        pub fn welcome() {
            print!("\x1B[2J\x1B[1;1H");
            stdout().flush().expect("Flush Error");
            println!("\n<==== TODOs Program ====>");
        }

        pub fn main_menu() -> u8 {
            println!("\n<--- Main Menu --->\n");
            println!("1> Add a new Note");
            println!("2> Display all Notes");
            println!("3> Delete a Note");
            println!("4> Exit Program\n");

            match take_input("\nOption:").as_str().parse::<u8>() {
                Ok(n) => {
                    return n;
                }
                Err(_) => panic!("Type Conversion Error"),
            };
        }

        pub fn new_note() -> (String, String) {
            let title = take_input("Note Title:");
            let desc = take_input("\nNote Description:");
            return (title, desc);
        }

        pub fn del_note() -> u8 {
            let id: String = take_input("Enter Note Id:");
            match id.as_str().parse::<u8>() {
                Ok(n) => {
                    return n;
                }
                Err(_) => {
                    return 0;
                }
            }
        }
    }

    pub struct Msg {}
    impl Msg {
        pub fn alert(txt: &str) {
            println!("\n=> [ {} ]\n", txt);
        }
        pub fn clear_alert(txt: &str) {
            console_clear();
            println!("\n=> [ {} ]\n", txt);
        }
        pub fn flash(txt: &str) {
            console_clear();
            super::menus::Msg::alert(txt);
            wait(1);
            console_clear();
        }
    }
}
