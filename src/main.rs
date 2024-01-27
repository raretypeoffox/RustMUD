// main.rs

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use futures::FutureExt;
use std::collections::HashMap;
use std::sync::Arc;
use std::io::Write; 

mod handler; // Include the handler module

use handler::process_player_input; // Import the handle_player_input function

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let player_manager = Arc::new(Mutex::new(PlayerManager::new()));

    let manager_clone = player_manager.clone();
    tokio::spawn(async move {
        loop {
            //tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            update_game_state(&manager_clone);
        }
    });

    loop {
        let (socket, _) = listener.accept().await?;
        let socket = Arc::new(Mutex::new(socket));  // Wrap the TcpStream in an Arc<Mutex<_>>.
        let player_manager_clone = player_manager.clone();

        let player_id = {
            let mut player_manager = player_manager_clone.lock().await;
            player_manager.add_player()
        };

        let socket_clone = Arc::clone(&socket);  // Clone the Arc<Mutex<TcpStream>> for the new task.

        tokio::spawn(async move {
            handle_player_input(socket_clone, player_id, player_manager_clone).await;
        });
        
        let socket_clone = Arc::clone(&socket);  // Clone the Arc<Mutex<TcpStream>> for the new task.
        let player_manager_clone = player_manager.clone();
        
        tokio::spawn(async move {
            handle_player_output(socket_clone, player_id, player_manager_clone).await;
        });
    }
}

fn update_game_state(player_manager: &Arc<Mutex<PlayerManager>>) {
    let mut player_manager = player_manager.lock();
    // Update game state here.
}

async fn handle_player_input(socket: Arc<Mutex<TcpStream>>, player_id: usize, player_manager: Arc<Mutex<PlayerManager>>) {
    let mut buffer = [0; 1024];
    loop {
        let mut socket = socket.lock().await;
        match socket.read(&mut buffer).await {
            Ok(0) => {
                // The client has closed the connection
                println!("Client disconnected");
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
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

async fn handle_player_output(socket: Arc<Mutex<TcpStream>>, player_id: usize, player_manager: Arc<Mutex<PlayerManager>>) {
    loop {
        let mut player_manager = player_manager.lock().await;
        if let Some(player) = player_manager.players.get_mut(&player_id) {
            if !player.output_buffer.is_empty() {
                println!("Writing to socket...");
                let mut socket = socket.lock().await;
                if let Err(e) = socket.write_all(&player.output_buffer).await {
                    eprintln!("Failed to write to socket: {}", e);
                    return;
                }

                // Flush the socket
                println!("Flushing socket...");
                if let Err(e) = socket.flush().await {
                    eprintln!("Failed to flush socket: {}", e);
                    return;
                }

                // Clear the output buffer
                player.output_buffer.clear();
            }
        }

        // Add a delay to prevent the loop from running too fast
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
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
