mod library;
use crate::library::tiktaktoe::game::TikTakToe;

fn main() {
    let mut game: TikTakToe = TikTakToe::new();

    loop {
        if game.play() && !game.play_again() {
            break;
        }
    }
}
