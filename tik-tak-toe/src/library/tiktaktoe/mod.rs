pub mod game {
    use std::io::{ Write, stdout, stdin };
    use rand::{ Rng, thread_rng };
    use crate::library::utils::utils::{ console_clear, wait };

    pub const RED: &str = "\x1b[31m";
    pub const BLUE: &str = "\x1b[34m";
    pub const RESET: &str = "\x1b[0m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const GREEN: &str = "\x1b[32m";

    #[derive(PartialEq, Clone, Copy)]
    pub enum Turn {
        PLAYER1 = 1,
        PLAYER2 = 2,
    }
    impl Turn {
        pub fn from_i32(num: i32) -> Self {
            match num {
                0 => Self::PLAYER1,
                _ => Self::PLAYER2,
            }
        }
    }

    pub struct TikTakToe {
        turn: Turn,
        board: [[[u8; 2]; 3]; 3],
    }
    impl TikTakToe {
        pub fn new() -> Self {
            let toss: i32 = thread_rng().gen_range(0..=1);
            Self {
                turn: Turn::from_i32(toss),
                board: [[[0; 2]; 3]; 3],
            }
        }

        pub fn take_input(&self) -> i8 {
            match &self.turn {
                Turn::PLAYER1 => print!("\n -⌲ {}Player-1:{} ", RED, RESET),
                Turn::PLAYER2 => print!("\n -⌲ {}Player-2:{} ", BLUE, RESET),
            }
            stdout().flush().expect("Flush Error");
            let mut opt: String = String::new();
            stdin().read_line(&mut opt).expect("Input Read Error");
            match opt.trim().parse::<i8>() {
                Ok(n) => {
                    return n;
                }
                Err(_) => {
                    return -1;
                }
            }
        }

        pub fn enter_to_continue(&mut self) {
            print!("\n\n");
            stdout().flush().expect("Flush Error");
            let mut opt: String = String::new();
            stdin().read_line(&mut opt).expect("Input Read Error");
        }

        pub fn update_board(&mut self, i: i8) -> bool {
            match i {
                -1 | 0 => {
                    return false;
                }
                n if n > 9 => {
                    return false;
                }
                _ => {
                    match self.board[((i - 1) / 3) as usize][((i - 1) % 3) as usize][1] {
                        n if n == (Turn::PLAYER1 as u8) || n == (Turn::PLAYER2 as u8) => {
                            return false;
                        }
                        _ => {
                            self.board[((i - 1) / 3) as usize][((i - 1) % 3) as usize][0] = 1;
                            self.board[((i - 1) / 3) as usize][((i - 1) % 3) as usize][1] =
                                self.turn as u8;
                            return true;
                        }
                    }
                }
            }
        }

        pub fn check_block(&mut self, x: usize, y: usize, player: Turn) -> bool {
            if self.board[x][y][0] == 1 && self.board[x][y][1] == (player as u8) {
                return true;
            } else {
                return false;
            }
        }

        pub fn check_player_victory(&mut self, player: Turn) -> bool {
            for i in 0..3 {
                if
                    self.check_block(i, 0, player) &&
                    self.check_block(i, 1, player) &&
                    self.check_block(i, 2, player)
                {
                    return true;
                } else if
                    self.check_block(0, i, player) &&
                    self.check_block(1, i, player) &&
                    self.check_block(2, i, player)
                {
                    return true;
                }
            }
            if
                self.check_block(0, 0, player) &&
                self.check_block(1, 1, player) &&
                self.check_block(2, 2, player)
            {
                return true;
            } else if
                self.check_block(0, 2, player) &&
                self.check_block(1, 1, player) &&
                self.check_block(2, 0, player)
            {
                return true;
            } else {
                return false;
            }
        }

        pub fn check_victory(&mut self) -> u8 {
            if self.check_player_victory(Turn::PLAYER1) {
                return Turn::PLAYER1 as u8;
            }
            if self.check_player_victory(Turn::PLAYER2) {
                return Turn::PLAYER2 as u8;
            }
            return 0;
        }

        pub fn check_draw(&mut self) -> bool {
            let mut flag: bool = false;
            for i in self.board {
                for j in i {
                    if j[0] == 0 && j[1] == 0 {
                        flag = true;
                    }
                }
            }
            return !flag;
        }

        pub fn game_reset(&mut self) {
            let toss: i32 = thread_rng().gen_range(0..=1);
            self.turn = Turn::from_i32(toss);
            self.board = [[[0; 2]; 3]; 3];
        }

        pub fn play_again(&mut self) -> bool {
            wait(1);
            self.game_reset();
            print!("\n -⌲ Play again? (Y):  ");
            stdout().flush().expect("Flush Error");
            let mut opt: String = String::new();
            stdin().read_line(&mut opt).expect("Input Read Error");
            println!("\n");
            match opt.trim() {
                n if n == "Y" || n == "y" => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }

        pub fn play(&mut self) -> bool {
            self.display_board();
            match self.check_victory() {
                n if n == (Turn::PLAYER1 as u8) => {
                    println!("\n 🎁 {}Player-1 WON !!!{} 🎁 \n", RED, RESET);
                    return true;
                }
                n if n == (Turn::PLAYER2 as u8) => {
                    println!("\n 🎁 {}Player-2 WON !!!{} 🎁 \n", BLUE, RESET);
                    return true;
                }
                _ => {
                    match self.check_draw() {
                        true => {
                            println!("\n 🔆 Draw Match 🔆 ");
                            self.enter_to_continue();
                            return true;
                        }
                        _ => {
                            match self.update_board(self.take_input()) {
                                true => {
                                    self.turn_toggle();
                                }
                                false => {
                                    println!("\n ❌ Invalid Input ❌");
                                    self.enter_to_continue();
                                }
                            }
                            return false;
                        }
                    }
                }
            }
        }

        pub fn turn_toggle(&mut self) {
            if self.turn == Turn::PLAYER1 {
                self.turn = Turn::PLAYER2;
            } else {
                self.turn = Turn::PLAYER1;
            }
        }

        pub fn get_char(&self, x: usize, y: usize) -> &'static str {
            match &self.board[x][y][1] {
                1 => "X",
                2 => "O",
                _ => " ",
            }
        }

        pub fn color_reset(&mut self) {
            print!("{}", RESET);
        }

        pub fn get_color(&self, x: usize, y: usize) -> &'static str {
            match &self.board[x][y][1] {
                1 => RED,
                2 => BLUE,
                _ => RESET,
            }
        }

        pub fn display_board(&mut self) {
            console_clear();
            println!(
                r"{}
  ++++++++++++++++++++
  + {}Tik-Tak-Toe Game{} +
  ++++++++++++++++++++

    {}
    -------------
    | {}{}{} | {}{}{} | {}{}{} |
    -------------
    | {}{}{} | {}{}{} | {}{}{} |
    -------------
    | {}{}{} | {}{}{} | {}{}{} |
    -------------
            ",
                GREEN,
                YELLOW,
                GREEN,
                YELLOW,
                self.get_color(0, 0),
                self.get_char(0, 0),
                YELLOW,
                self.get_color(0, 1),
                self.get_char(0, 1),
                YELLOW,
                self.get_color(0, 2),
                self.get_char(0, 2),
                YELLOW,
                self.get_color(1, 0),
                self.get_char(1, 0),
                YELLOW,
                self.get_color(1, 1),
                self.get_char(1, 1),
                YELLOW,
                self.get_color(1, 2),
                self.get_char(1, 2),
                YELLOW,
                self.get_color(2, 0),
                self.get_char(2, 0),
                YELLOW,
                self.get_color(2, 1),
                self.get_char(2, 1),
                YELLOW,
                self.get_color(2, 2),
                self.get_char(2, 2),
                YELLOW
            );
            self.color_reset();
        }
    }
}
