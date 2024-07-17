use std::process;

pub const HEIGHT: u32 = 20;
pub const WIDTH: u32 = 20;

#[derive(PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct Game {
    pub snake: Vec<Point>,
    pub food: Point,
    pub direction: Direction,
    pub score: u32
}

impl Game {
    pub fn new() -> Game {
        Game {
            snake: vec![Point { x: WIDTH as i32 / 2, y: HEIGHT as i32 / 2 }],
            food: Point{ x:10, y:15 },
            direction: Direction::Up,
            score: 0
        }
    }

    pub fn generate_food() -> Point {
        Point {
            x: (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() % WIDTH as u64) as i32,
            y: (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() % HEIGHT as u64) as i32,
        }
    }

    pub fn move_snake(&mut self) {
        let head = self.snake.first().unwrap();
        let new_head = match &self.direction {
            Direction::Up => Point { x: head.x, y: (head.y - 1 + HEIGHT as i32) % HEIGHT as i32 },
            Direction::Down => Point { x: head.x, y: (head.y + 1) % HEIGHT as i32 },
            Direction::Left => Point { x: (head.x - 1 + WIDTH as i32) % WIDTH as i32, y: head.y },
            Direction::Right => Point { x: (head.x + 1) % WIDTH as i32, y: head.y },
        };
        if self.snake.contains(&new_head) {
            process::exit(1);
        }
        self.snake.insert(0, new_head);

        if self.snake[0] == self.food {
            self.score += 1;
            self.food = Game::generate_food();
        } else {
            self.snake.pop();
        }
    }

    pub fn draw(&self) {


        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let point = Point { x: x as i32, y: y as i32};
                if self.snake.contains(&point) {
                    print!("@");
                } else if self.food == point {
                    print!("x");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}