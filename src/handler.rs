// handler.rs

use tokio::sync::Mutex;
use std::sync::Arc;
use super::PlayerManager; // Assuming PlayerManager is defined in the parent module
use std::io;


pub async fn process_player_input(input: &[u8], player_id: usize, player_manager: Arc<Mutex<PlayerManager>>) -> io::Result<()> {
    println!("Handling player input: {}", String::from_utf8_lossy(input));

    let mut player_manager = player_manager.lock().await;
    if let Some(player) = player_manager.players.get_mut(&player_id) {
        println!("Appending to player's output_buffer");
        player.append_to_output_buffer(input);
    }

    Ok(())
}
