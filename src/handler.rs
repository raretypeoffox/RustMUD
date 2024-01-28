// handler.rs

use std::io;
use super::PlayerManager;

enum Command {
    Chat,
    North,
    Echo,
    // Add other commands here...
}

fn string_to_command(s: &str) -> Option<Command> {
    match s {
        "n" => Some(Command::North),
        _ => {
            if s.len() > 1 {
                if "north".starts_with(s) {
                    Some(Command::North)
                } else if "chat".starts_with(s) {
                    Some(Command::Chat)
                } else if "echo".starts_with(s) {
                    Some(Command::Echo)
                }
                // Add other string-command mappings here...
                else {
                    None
                }
            } else {
                None
            }
        }
    }
}

fn chat_command(player_manager: &mut PlayerManager, _player_id: usize, argument: &str) {
    player_manager.send_global_message(argument.to_string());
}

fn echo_command(player_manager: &mut PlayerManager, player_id: usize, argument: &str) {
    player_manager.send_message(player_id, format!("You said: {}", argument));
}


fn north_command(_player_manager: &mut PlayerManager, _player_id: usize, _argument: &str) {
    // Implement the north command here...
}


fn parse_input(input: &str) -> (Option<Command>, String) {
    let mut words = input.split_whitespace();
    let command = words.next().map(|s| string_to_command(&s.to_lowercase())).flatten();
    let argument = words.collect::<Vec<&str>>().join(" ");
    (command, argument)
}

pub fn process_player_input(player_manager: &mut PlayerManager, player_id: usize,) -> io::Result<()> {

    let input = player_manager.read_player_input(player_id);

    println!("Handling player input: {}", input);

    let (command, argument) = parse_input(&input);

    match command {
        Some(Command::Chat) => chat_command(player_manager, player_id, &argument),
        Some(Command::North) => north_command(player_manager, player_id, &argument),
        Some(Command::Echo) => echo_command(player_manager, player_id, &argument),
        // Add other command matches here...
        None => player_manager.send_message(player_id, "I don't understand that command.".to_string()),
    }

    Ok(())
}
