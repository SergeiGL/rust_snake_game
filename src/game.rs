use crossterm::event::{self, Event, KeyCode};

use rand::{seq::SliceRandom, thread_rng};
use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};


#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Coordinate {
    pub x: u8,
    pub y: u8,
}


#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum Direction { UP, DOWN, LEFT, RIGHT }

impl Direction {
    pub fn get_opposite(self) -> Direction {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
        }
    }
}


pub struct Snake {
    pub direction: Direction,
    pub body: VecDeque<Coordinate>,
}

impl Snake {
    pub fn new(board_size: usize) -> Snake {
        Snake {
            direction: Direction::LEFT,
            body: {
                let mut res = VecDeque::with_capacity(board_size * board_size + 3);
                res.push_back(Coordinate { x: 0, y: 0 });
                res.push_back(Coordinate { x: 1, y: 0 });
                res
            },
        }
    }


    fn get_head(&self) -> &Coordinate {
        self.body.front().unwrap() // head always exists as length > 0
    }

    fn rm_tail(&mut self) {
        self.body.pop_back().unwrap(); // tail always exists as length > 0
    }
}


pub struct Game {
    board_size: usize,
    snake: Snake,
    food_pos: Coordinate,
    pub freeze_time_ms: u64,
}


impl Game {
    pub fn clear_terminal() {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn get_user_key(&self) -> Result<Direction, &'static str> {
        print!("Please enter your move (w/a/s/d): ");
        stdout().flush().unwrap();

        let mut last_input_time = Instant::now();
        let debounce_duration = Duration::from_millis(self.freeze_time_ms);

        loop {
            if event::poll(Duration::from_millis(10)).unwrap() {
                if let Event::Key(key_event) = event::read().unwrap() {
                    let current_time = Instant::now();
                    if current_time.duration_since(last_input_time) < debounce_duration {
                        continue;
                    }

                    last_input_time = current_time;

                    let result = match key_event.code {
                        KeyCode::Char('w') => Ok(Direction::UP),
                        KeyCode::Char('a') => Ok(Direction::LEFT),
                        KeyCode::Char('s') => Ok(Direction::DOWN),
                        KeyCode::Char('d') => Ok(Direction::RIGHT),
                        KeyCode::Char('c') if key_event.modifiers.contains(event::KeyModifiers::CONTROL) => {
                            Err("User interrupted")
                        }
                        _ => continue,
                    };

                    Self::clear_terminal();
                    return result;
                }
            }
        }
    }

    pub fn new(board_size: usize) -> Game {
        Game {
            board_size,
            snake: Snake::new(board_size),
            food_pos: Coordinate { x: 0, y: 1 },
            freeze_time_ms: 200,
        }
    }

    fn update_food_coord(&mut self) {
        let mut available_positions = Vec::with_capacity(self.board_size * self.board_size - self.snake.body.len() + 1);

        for x in 0..self.board_size {
            for y in 0..self.board_size {
                let coord = Coordinate { x: x as u8, y: y as u8 };
                if !self.snake.body.contains(&coord) {
                    available_positions.push(coord);
                }
            }
        }

        self.food_pos = available_positions.choose(&mut thread_rng()).cloned().unwrap();
    }


    pub fn change_snake_dir(&mut self, new_dir: Direction) {
        self.snake.direction = new_dir;
    }

    pub fn step(&mut self) -> bool {
        let Coordinate { x: x_head, y: y_head } = *self.snake.get_head();

        let next_coord = match self.snake.direction {
            Direction::UP if y_head as usize + 1 < self.board_size => Coordinate { x: x_head, y: y_head + 1 },
            Direction::DOWN if y_head as i64 - 1 >= 0 => Coordinate { x: x_head, y: y_head - 1 },
            Direction::LEFT if x_head as i64 - 1 >= 0 => Coordinate { x: x_head - 1, y: y_head },
            Direction::RIGHT if x_head as usize + 1 < self.board_size => Coordinate { x: x_head + 1, y: y_head },
            _ => {
                println!("GAME OVER (out of bounce)");
                return false;
            }
        };

        if self.snake.body.contains(&next_coord) {
            println!("GAME OVER (collision with the snake)");
            return false;
        }

        let eat_food = next_coord == self.food_pos;

        match eat_food {
            true => {
                if self.snake.body.len() + 1 >= self.board_size * self.board_size {
                    println!("WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!WIN!");
                    return false;
                } else {
                    self.snake.body.push_front(self.food_pos);
                    self.update_food_coord();
                }
            }
            false => {
                self.snake.rm_tail();
                self.snake.body.push_front(next_coord);
            }
        };


        true
    }

    pub fn get_current_dir(&self) -> Direction {
        self.snake.direction
    }

    pub fn display_state(&self) {
        let mut board = vec![vec!['*'; self.board_size]; self.board_size];

        for segment in &self.snake.body {
            board[self.board_size - 1 - segment.y as usize][segment.x as usize] = 'S';
        }
        let head = self.snake.body.front().unwrap(); // always exists as len >0
        board[self.board_size - 1 - head.y as usize][head.x as usize] = 'H';


        board[self.board_size - 1 - self.food_pos.y as usize][self.food_pos.x as usize] = 'F';

        for row in board.iter() {
            for cell in row.iter() {
                print!("{} ", cell);
            }
            println!();
        }
    }
}