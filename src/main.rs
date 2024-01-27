use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use futures::FutureExt;
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let player_manager = Arc::new(Mutex::new(PlayerManager::new()));

    let manager_clone = player_manager.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
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
            handle_connection(socket_clone, player_id, player_manager_clone).await;
        });
    }
}

fn update_game_state(player_manager: &Arc<Mutex<PlayerManager>>) {
    let mut player_manager = player_manager.lock();
    // Update game state here.
}

async fn handle_connection(socket: Arc<Mutex<TcpStream>>, player_id: usize, player_manager: Arc<Mutex<PlayerManager>>) {
    let mut buffer = [0; 1024];

    loop {
        tokio::select! {
            bytes_read = async {
                let mut socket_locked = socket.lock().await;
                socket_locked.read(&mut buffer).await
            }.boxed() => {
                let bytes_read = match bytes_read {
                    Ok(0) => {
                        println!("Client disconnected.");
                        let mut player_manager = player_manager.lock().await;
                        player_manager.remove_player(player_id);
                        return;
                    }
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to read from socket: {}", e);
                        return;
                    }
                };

                let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received from client: {}", message);

                let mut player_manager = player_manager.lock().await;
                if let Some(player) = player_manager.players.get_mut(&player_id) {
                    player.append_to_output_buffer(&buffer[..bytes_read]);
                }
            }
            _ = async {
                let mut player_manager = player_manager.lock().await;
                if let Some(player) = player_manager.players.get_mut(&player_id) {
                    if !player.output_buffer.is_empty() {
                        println!("Sending to client: {}", String::from_utf8_lossy(&player.output_buffer));
                        if let Err(e) = socket.lock().await.write_all(&player.output_buffer).await {
                            eprintln!("Failed to write to socket: {}", e);
                            return;
                        }
                        player.output_buffer.clear();
                    }
                }
            } => {}
        }
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
