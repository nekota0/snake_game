use snake::*;
use std::io::{self, Read};
use std::time::{Duration, Instant};
use std::thread;

#[tokio::main]
async fn main() {
    let mut game = Game::new();
    let mut last_update = Instant::now();
    let mut stdin = io::stdin();
    print!("\x1B[2J\x1B[1;1H");
    
    loop {
        if last_update.elapsed() >= Duration::from_millis(200) {
            game.move_snake();
            game.draw();
            last_update = Instant::now();
        }

        let mut input = [0];
        if stdin.read(&mut input).is_ok() {
            match input[0] as char {
                'w' => game.direction = Direction::Up,
                's' => game.direction = Direction::Down,
                'a' => game.direction = Direction::Left,
                'd' => game.direction = Direction::Right,
                'q' => break,
                _ => {}
            }
        }

        print!("\x1B[2J\x1B[1;1H");

        thread::sleep(Duration::from_millis(50));
    }
}