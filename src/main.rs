mod game;

use game::*;
use std::process;


fn main() {
    let mut game = Game::new(5);
    
    let mut is_game_running = true;

    while is_game_running {
        game.display_state();

        let key_pressed = game.get_user_key();

        match key_pressed {
            Ok(new_dir) if new_dir.get_opposite() == game.get_current_dir() => {
                println!("Opposite Direction!Opposite Direction!Opposite Direction!");
                continue;
            }
            Ok(key_pressed) => game.change_snake_dir(key_pressed),
            Err(e) if e == "User interrupted" => process::exit(0),
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        is_game_running = game.step();
    }
}
