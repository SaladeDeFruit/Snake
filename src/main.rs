use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::style::Stylize;
use crossterm::terminal::{self};
use crossterm::{cursor, execute, queue, style};
use rand::Rng;
use std::process;
use std::{collections::VecDeque, io::stdout, io::Write, thread, time::Duration};

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone)]
struct Snake {
    body: VecDeque<(i32, i32)>,
    direction: Direction,
}

fn make_snake(c: (i32, i32), direction: Direction) -> Snake {
    let mut body: VecDeque<(i32, i32)> = VecDeque::new();
    body.push_back((c.0, c.1));
    Snake {
        body: body,
        direction: direction,
    }
}

impl Snake {
    // sets the snake to the next position without removing its excess tail
    fn next_position(&mut self, size: i32) -> Result<(), &str>{
        match self.body[0] {
            (x, y) => match self.direction {
                Direction::Left => {
                    if x > 0 {

                        for i in 1..self.body.len() {
                            if x - 1 == self.body[i].0 && y == self.body[i].1 {
                                return Err("eat-myself");
                            }
                        }

                        self.body.push_front((x - 1, y));
                        return Ok::<(), &str>(())
                    }
                    Err("wall")
                }
                Direction::Right => {
                    if x < size - 1 {

                        for i in 1..self.body.len() {
                            if x + 1 == self.body[i].0 && y == self.body[i].1 {
                                return Err("eat-myself");
                            }
                        }

                        self.body.push_front((x + 1, y));
                        return Ok::<(), &str>(())
                    }
                    Err("wall")
                }
                Direction::Up => {
                    if y > 0 {

                        for i in 1..self.body.len() {
                            if x == self.body[i].0 && y - 1 == self.body[i].1 {
                                return Err("eat-myself");
                            }
                        }

                        self.body.push_front((x, y - 1));
                        return Ok::<(), &str>(())
                    }
                    Err("wall")
                }
                Direction::Down => {
                    if y < size - 1 {


                        for i in 1..self.body.len() {
                            if x == self.body[i].0 && y + 1 == self.body[i].1 {
                                return Err("eat-myself");
                            }
                        }

                        self.body.push_front((x, y + 1));
                        return Ok::<(), &str>(())
                    }
                    Err("wall")
                }
            },
        }
        //self.body.pop_back();
    }

    // removes the excess tail
    fn move_snake(&mut self, size: i32) -> (){
        match self.next_position(size) {
            Ok(_) => {self.body.pop_back();},
            Err(_) => {process::exit(0x0100);},
        }
    }

    // grows thw snake
    fn grow(&mut self, size: i32) -> () {
        let _ = self.next_position(size);
    }
}

enum Colour {
    Green,
    DarkGreen,
    White,
    Red,
}

fn print_map(player: &Snake, apple: (i32, i32), size: i32) -> () {
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    let heigth = terminal::size().unwrap().0;
    let width = terminal::size().unwrap().1;
    //alternate colour
    let mut c = 0;
    let mut colour = Colour::White;
    for i in 0..size {
        for j in 0..size {
            if apple.0 == i && apple.1 == j {
                colour = Colour::Red;
            }
            for tail in &player.body {
                match tail {
                    (x, y) => {
                        if x == &j && y == &i {
                            if c % 2 == 0 {
                                colour = Colour::Green;
                            } else {
                                colour = Colour::DarkGreen;
                            }
                            c += 1;
                        }
                    }
                }
            }
            queue!(
                stdout,
                cursor::MoveTo(
                    j as u16 + heigth / 2 - (size as u16) / 2,
                    i as u16 + width / 2 - (size as u16) / 2
                )
            )
            .unwrap();
            match colour {
                Colour::DarkGreen => {
                    queue!(stdout, style::PrintStyledContent("O".dark_green())).unwrap()
                }
                Colour::Green => queue!(stdout, style::PrintStyledContent("O".green())).unwrap(),
                Colour::White => queue!(stdout, style::PrintStyledContent(".".white())).unwrap(),
                Colour::Red => queue!(stdout, style::PrintStyledContent("o".red())).unwrap(),
            };
            stdout.flush().unwrap();
            colour = Colour::White;
        }
        //•◦▒▓
        // resets the color for the board
    }
    queue!(stdout, cursor::MoveTo(0, 0)).unwrap();
}

fn main() {
    let size = 15;
    let mut rng = rand::thread_rng();
    let mut player = make_snake((0, 0), Direction::Right);
    let mut apple = (rng.gen_range(0..size - 1), rng.gen_range(0..size - 1));
    let _ = terminal::enable_raw_mode();
    loop {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Key(event) => match event.code {
                    KeyCode::Right => player.direction = Direction::Right,
                    KeyCode::Left => player.direction = Direction::Left,
                    KeyCode::Up => player.direction = Direction::Up,
                    KeyCode::Down => player.direction = Direction::Down,
                    _ => {}
                },
                _ => {}
            }
        }
        if player.body[0].0 == apple.1 && player.body[0].1 == apple.0 {
            player.grow(size);
            apple = (rng.gen_range(0..size - 1), rng.gen_range(0..size - 1));
        } else {
            player.move_snake(size);
        }
        print_map(&player, apple, size);
        thread::sleep(Duration::from_millis(150));
        queue!(stdout(), cursor::MoveTo(0, 0)).unwrap();
    }
}

// cargu run ./Programmation/Rust/snake
