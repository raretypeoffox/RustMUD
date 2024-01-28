// handler.rs

use std::pin::Pin;
use std::future::Future;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::io;
use std::collections::HashMap;
use super::PlayerManager;

type CommandFn = Arc<dyn Fn(String, Arc<Mutex<PlayerManager>>, usize) -> Pin<Box<dyn Future<Output = io::Result<()>> + Send>> + Send + Sync>;

struct CommandHandler {
    commands: HashMap<String, CommandFn>,
}

impl CommandHandler {
    fn new() -> Self {
        let mut commands = HashMap::new();
        Self { commands }
    }

    fn add_command<F, Fut>(&mut self, name: String, function: F) 
    where
        F: Fn(String, Arc<Mutex<PlayerManager>>, usize) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = io::Result<()>> + Send + 'static,
    {
        let function = move |argument, player_manager, player_id| {
            let future: Pin<Box<dyn Future<Output = io::Result<()>> + Send>> = Box::pin(function(argument, player_manager, player_id));
            future
        };
        self.commands.insert(name, Arc::new(function) as CommandFn);
    }

    async fn execute_command(&self, command: String, argument: String, player_manager: Arc<Mutex<PlayerManager>>, player_id: usize) -> io::Result<()> {
        if let Some(command_fn) = self.commands.get(&command) {
            (command_fn)(argument, player_manager, player_id).await?;
        } else {
            send_message_to_player(player_manager, player_id, "Unknown command\n").await?;
        }
        Ok(())
    }
}

async fn send_message_to_player(player_manager: Arc<Mutex<PlayerManager>>, player_id: usize, message: &str) -> io::Result<()> {
    let mut player_manager = player_manager.lock().await;
    if let Some(player) = player_manager.players.get_mut(&player_id) {
        player.append_to_output_buffer(message.as_bytes());
    }
    Ok(())
}

fn parse_input(input: &str) -> (String, String) {
    let mut words = input.split_whitespace();
    let command = words.next().unwrap_or("").to_lowercase();
    let argument = words.collect::<Vec<&str>>().join(" ");
    (command, argument)
}

pub async fn process_player_input(input: &[u8], player_id: usize, player_manager: Arc<Mutex<PlayerManager>>) -> io::Result<()> {
    let input = String::from_utf8_lossy(input);
    println!("Handling player input: {}", input);

    let (command, argument) = parse_input(&input);

    let mut command_handler = CommandHandler::new();
    command_handler.add_command("chat".to_string(), chat_command);

    command_handler.execute_command(command, argument, player_manager.clone(), player_id).await?;

    Ok(())
}

async fn chat_command(argument: String, player_manager: Arc<Mutex<PlayerManager>>, player_id: usize) -> io::Result<()> {
    println!("Executing chat command with argument: {}", argument);
    let mut player_manager = player_manager.lock().await;
    if let Some(player) = player_manager.players.get_mut(&player_id) {
        player.append_to_output_buffer(format!("{}\n", argument).as_bytes());
    }
    Ok(())
}


// pub async fn process_player_input(input: &[u8], player_id: usize, player_manager: Arc<Mutex<PlayerManager>>) -> io::Result<()> {
//     println!("Handling player input: {}", String::from_utf8_lossy(input));

//     let mut player_manager = player_manager.lock().await;;
//     if let Some(player) = player_manager.players.get_mut(&player_id) {
//         println!("Appending to player's output_buffer");
//         player.append_to_output_buffer(input);
//     }

//     Ok(())
// }

// pub async fn process_player_input(input: &[u8], player_id: usize, player_manager: Arc<Mutex<PlayerManager>>) -> io::Result<()> {
//     println!("Handling player input: {}", String::from_utf8_lossy(input));

//     // Broadcast the player's input to all players
//     broadcast_message(input, player_manager.clone()).await?;

//     Ok(())
// }

// pub async fn broadcast_message(message: &[u8], player_manager: Arc<Mutex<PlayerManager>>) -> io::Result<()> {
//     let mut player_manager = player_manager.lock().await;
//     for player in player_manager.players.values_mut() {
//         println!("Appending to player's output_buffer");
//         player.append_to_output_buffer(message);

//     }

//     Ok(())
// }