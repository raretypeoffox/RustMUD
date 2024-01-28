// handler.rs

use tokio::sync::Mutex;
use std::sync::Arc;
use super::PlayerManager; // Assuming PlayerManager is defined in the parent module
use std::io;


// pub async fn process_player_input(input: &[u8], player_id: usize, player_manager: Arc<Mutex<PlayerManager>>) -> io::Result<()> {
//     println!("Handling player input: {}", String::from_utf8_lossy(input));

//     println!("Attempting to lock player_manager in process_player_output");
//     let mut player_manager = player_manager.lock().await;
//     println!("PlayerManager locked in process_player_output");
//     if let Some(player) = player_manager.players.get_mut(&player_id) {
//         println!("Appending to player's output_buffer");
//         player.append_to_output_buffer(input);
//     }

//     Ok(())
// }

pub async fn process_player_input(input: &[u8], player_id: usize, player_manager: Arc<Mutex<PlayerManager>>) -> io::Result<()> {
    println!("Handling player input: {}", String::from_utf8_lossy(input));

    // Broadcast the player's input to all players
    broadcast_message(input, player_manager.clone()).await?;

    Ok(())
}

pub async fn broadcast_message(message: &[u8], player_manager: Arc<Mutex<PlayerManager>>) -> io::Result<()> {
    let mut player_manager = player_manager.lock().await;
    for player in player_manager.players.values_mut() {
        println!("Appending to player's output_buffer");
        player.append_to_output_buffer(message);

    }

    Ok(())
}