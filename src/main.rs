use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let player_manager = Arc::new(Mutex::new(PlayerManager::new()));

    loop {
        let (socket, _) = listener.accept().await?;
        let manager_clone = player_manager.clone();

        let player_id = {
            let mut manager = manager_clone.lock().unwrap();
            manager.add_player()
        };

        tokio::spawn(async move {
            handle_connection(socket, player_id, manager_clone).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream, player_id: usize, manager: Arc<Mutex<PlayerManager>>) {
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = match socket.read(&mut buffer).await {
            Ok(0) => {
                println!("Client disconnected.");
                let mut manager = manager.lock().unwrap();
                manager.remove_player(player_id);
                return;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                return;
            }
        };

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
