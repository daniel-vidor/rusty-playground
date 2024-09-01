use std::io::{self, Write};
use std::process;

struct GameState {
    player: Player,
    map: Map,
}

// Origin (0, 0) is the top-left
#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

struct Player {
    pos: Position,
}

struct Map {
    max_x: i32,
    max_y: i32,
}

enum Command {
    Move { x: i32, y: i32 },
    Wait,
    Quit,
    Unknown,
}

fn main() {
    let player = Player { pos: Position { x: 4, y: 4 } };
    let map = Map { max_x: 12, max_y: 8 };
    let mut gs = GameState { player: player, map: map };

    println!("Welcome to Rustlike, a Roguelike developed in Rust.");

    loop {
        print!("Enter some input: ");
        // Flush to ensure the prompt is displayed before waiting for input
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let trimmed_input = input.trim();
        let cmd = parse_input(trimmed_input);
        process_command(cmd, &mut gs);

        draw_screen(&gs);
    }
}

fn parse_input(input: &str) -> Command {
    match input.trim().to_lowercase().as_str() {
        "left" | "h" => Command::Move { x: -1, y: 0 },
        "down" | "j" => Command::Move { x: 0, y: 1 },
        "up" | "k" => Command::Move { x: 0, y: -1 },
        "right" | "l" => Command::Move { x: 1, y: 0 },
        "." => Command::Wait,
        "exit" | "quit" => Command::Quit,
        _ => Command::Unknown,
    }
}

fn process_command(cmd: Command, gs: &mut GameState) {
    match cmd {
        Command::Move { x, y } => process_player_move(x, y, gs),
        Command::Wait => println!("Waiting."),
        Command::Quit => process_exit(),
        Command::Unknown => println!("Unrecognised command."),
    }
}

fn process_player_move(x: i32, y: i32, gs: &mut GameState) {
    let old_pos = gs.player.pos;
    let new_pos = Position {
        x: old_pos.x + x,
        y: old_pos.y + y,
    };
    
    println!("Player moving from [{}, {}] to [{}, {}]...", old_pos.x, old_pos.y, new_pos.x, new_pos.y);
    
    if is_pos_in_bounds(&new_pos, gs.map.max_x, gs.map.max_y) {
        gs.player.pos = new_pos;
    } else {
        println!("Cannot move to position [{}, {}]; it is out of bounds.", new_pos.x, new_pos.y);
    }
}

fn is_pos_in_bounds(pos: &Position, max_x: i32, max_y: i32) -> bool {
    pos.x <= max_x && pos.x >= 0 && pos.y <= max_y && pos.y >= 0
}

fn process_exit() {
    println!("Exiting...");
    process::exit(1);
}

fn draw_screen(gs: &GameState) {
    for y in 0..gs.map.max_y + 1 {
        for x in 0..gs.map.max_x + 1 {
            if gs.player.pos.x == x && gs.player.pos.y == y {
                print!("@");
            } else {
                print!(".");
            }
        }

        println!("");
    }
}
