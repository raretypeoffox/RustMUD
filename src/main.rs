use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
        let player_manager_clone = player_manager.clone();

        let player_id = {
            let mut player_manager = player_manager_clone.lock().unwrap();
            player_manager.add_player()
        };

        tokio::spawn(async move {
            handle_connection(socket, player_id, player_manager_clone).await;
        });
    }
}

fn update_game_state(player_manager: &Arc<Mutex<PlayerManager>>) {
    let mut player_manager = player_manager.lock().unwrap();
    // Update game state here.
}

async fn handle_connection(mut socket: TcpStream, player_id: usize, player_manager: Arc<Mutex<PlayerManager>>) {
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = match socket.read(&mut buffer).await {
            Ok(0) => {
                println!("Client disconnected.");
                let mut player_manager = player_manager.lock().unwrap();
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

        if let Err(e) = socket.write_all(&buffer[..bytes_read]).await {
            eprintln!("Failed to write to socket: {}", e);
            return;
        }
    }
}

struct Player {
    // Player attributes here.
    // TcpStream is not stored here anymore.
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

        let player = Player { /* Initialize player attributes here */ };
        self.players.insert(id, player);

        id
    }

    fn remove_player(&mut self, id: usize) {
        self.players.remove(&id);
    }

    // Implement other methods to manage players, such as broadcast_message, etc.
}
