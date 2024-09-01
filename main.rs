use std::io::{self, Write};

struct Player {
    x: i32,
    y: i32,
}

enum Command {
    Move { x: i32, y: i32 },
    Wait,
    Quit,
    Unknown,
}

fn main() {
    let max_x = 20;
    let max_y = 15;
    let player = Player { x: 4, y: 6 };
    
    println!("Welcome to Rustlike, a Roguelike developed in Rust.");
    
    loop {
        print!("Enter some input: ");
        // Flush to ensure the prompt is displayed before waiting for input
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let trimmed_input = input.trim();
        let cmd = parse_input(trimmed_input);
        process_command(cmd);

        draw_screen(max_x, max_y, &player);
    }
}

fn parse_input(input: &str) -> Command {
    match input.trim().to_lowercase().as_str() {
        "left" | "h" => Command::Move { x: -1, y: 0 },
        "down" | "j" => Command::Move { x: 0, y: -1 },
        "up" | "k" => Command::Move { x: 0, y: 1 },
        "right" | "l" => Command::Move { x: 1, y: 0 },
        "." => Command::Wait,
        "exit" | "quit" => Command::Quit,
        _ => Command::Unknown,
    }
}

fn process_command(cmd: Command) {
    match cmd {
        Command::Move {x, y} => process_player_move(x, y),
        Command::Wait => println!("Waiting."),
        Command::Quit => println!("Exiting..."),
        Command::Unknown => println!("Unrecognised command."),
    }
}

fn process_player_move(x: i32, y: i32) {
    println!("Moving to {}, {}", x, y);
}

fn draw_screen(max_x: i32, max_y: i32, player: &Player) {
    for y in 0..max_y {
        for x in 0..max_x {
            if player.x == x && player.y == y {
                print!("@");
            } else {
                print!(".");
            }
        }
        
        println!("");
    }
}
