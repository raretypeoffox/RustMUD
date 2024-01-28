// main.rs

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

mod handler; // Include the handler module

use handler::process_player_input; // Import the handle_player_input function

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let player_manager = Arc::new(Mutex::new(PlayerManager::new()));

    let manager_clone = player_manager.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            update_game_state(&manager_clone);
        }
    });

    loop {
        let (socket, _) = listener.accept().await?;

        // Split the socket into a read half and a write half
        let (socket_reader, socket_writer) = socket.into_split();

        // Wrap the halves in Arc<Mutex<>> so they can be shared safely
        let socket_reader = Arc::new(Mutex::new(socket_reader));
        let socket_writer = Arc::new(Mutex::new(socket_writer));

        let player_manager_clone = player_manager.clone();

        let player_id = {
            let mut player_manager = player_manager_clone.lock().await;
            player_manager.add_player()
        };

        // Spawn the input handling task
        let socket_reader_clone = Arc::clone(&socket_reader);
        let player_manager_clone = player_manager.clone();
        tokio::spawn(async move {
            handle_player_input(socket_reader_clone, player_id, player_manager_clone).await;
        });

        // Spawn the output handling task
        let socket_writer_clone = Arc::clone(&socket_writer);
        let player_manager_clone = player_manager.clone();
        tokio::spawn(async move {
            handle_player_output(socket_writer_clone, player_id, player_manager_clone).await;
        });
    }
}

fn update_game_state(player_manager: &Arc<Mutex<PlayerManager>>) {
    let mut player_manager = player_manager.lock();
    // ... existing code ...
}

async fn handle_disconnection(player_id: usize, player_manager: Arc<Mutex<PlayerManager>>) {
    println!("Client disconnected");
    let mut player_manager = player_manager.lock().await;
    //player_manager.players.remove(&player_id);
    player_manager.remove_player(player_id);
}

async fn handle_player_input(socket_reader: Arc<Mutex<OwnedReadHalf>>, player_id: usize, player_manager: Arc<Mutex<PlayerManager>>) {
    let mut buffer = [0; 1024];
    loop {
        let mut socket_reader = socket_reader.lock().await;
        match socket_reader.read(&mut buffer).await {
            Ok(0) => {
                handle_disconnection(player_id, player_manager.clone()).await;
                return;
            }
            Ok(bytes_read) => {
                let message = buffer[..bytes_read].to_vec();

                // Process the player's input
                if let Err(e) = process_player_input(&message, player_id, player_manager.clone()).await {
                    eprintln!("Failed to process player input: {}", e);
                    return;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                return;
            }
        }

        // Add a delay to prevent the loop from running too fast
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
}

async fn handle_player_output(socket_writer: Arc<Mutex<OwnedWriteHalf>>, player_id: usize, player_manager: Arc<Mutex<PlayerManager>>) {
    loop {
        let player_manager_clone = player_manager.clone();
        println!("Attempting to lock player_manager in handle_player_output");
        let mut player_manager = player_manager.lock().await;
        println!("PlayerManager locked in handle_player_output");
        if let Some(player) = player_manager.players.get_mut(&player_id) {
            if !player.output_buffer.is_empty() {
                println!("Writing to socket: {}", String::from_utf8_lossy(&player.output_buffer));
                let mut socket_writer = socket_writer.lock().await;
                match socket_writer.write_all(&player.output_buffer).await {
                    Ok(_) => {
                        // Flush the socket
                        if let Err(e) = socket_writer.flush().await {
                            eprintln!("Failed to flush socket: {}", e);
                            handle_disconnection(player_id, player_manager_clone).await;
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to write to socket: {}", e);
                        handle_disconnection(player_id, player_manager_clone).await;
                        return;
                    }
                }

                // Clear the output buffer
                player.output_buffer.clear();
                println!("Releasing socket lock in handle_player_output");
            }
        } else {
            // Player not found in player_manager, possibly already removed due to disconnection
            return;
        }

        // Add a delay to prevent the loop from running too fast
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        println!("Releasing PlayerManager lock in handle_player_output");
    }
}

struct Player {
    // Player attributes here.
    output_buffer: Vec<u8>,
}

impl Player {
    // Other methods...

    fn append_to_output_buffer(&mut self, data: &[u8]) {
        self.output_buffer.extend_from_slice(data);
    }
}

struct PlayerManager {
    players: HashMap<usize, Player>,
    unique_id_counter: usize,
}

impl PlayerManager {
    fn new() -> PlayerManager {
        PlayerManager {
            players: HashMap::new(),
            unique_id_counter: 1,
        }
    }

    fn add_player(&mut self) -> usize {
        let id = self.unique_id_counter;
        self.unique_id_counter += 1;

        let player = Player { output_buffer: Vec::new() };
        self.players.insert(id, player);

        id
    }

    fn remove_player(&mut self, id: usize) {
        self.players.remove(&id);
    }

    // Implement other methods to manage players, such as broadcast_message, etc.
}
