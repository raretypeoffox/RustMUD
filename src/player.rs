
use crate::Conn;
use crate::Sex;
use crate::Race;
use crate::Origin;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::collections::HashMap;
use super::consts::constants::*;

pub struct Player {
    pub addr: SocketAddr,
    pub stream: TcpStream,
    pub input_buffer: Vec<u8>,
    pub output_buffer: Vec<u8>,
    pub connection_status: Conn,

    pub character_name: String,
    pub sex: Sex,
    pub race: Race,
    pub origin: Origin,
}

impl Player {
    // Other methods...

    pub fn append_to_input_buffer(&mut self, data: &[u8]) {
        self.input_buffer.extend_from_slice(data);
    }

    pub fn append_to_output_buffer(&mut self, data: String) {
        self.output_buffer.extend_from_slice(data.as_bytes());
    }

    pub fn read_input_buffer(&mut self) -> String {
        let mut input_buffer = Vec::new();
        std::mem::swap(&mut input_buffer, &mut self.input_buffer);
        String::from_utf8_lossy(&input_buffer).to_string()
    }
}

pub struct PlayerManager {
    pub players: HashMap<usize, Player>,
    pub unique_id_counter: usize,
}

impl PlayerManager {
    pub fn new() -> PlayerManager {
        PlayerManager {
            players: HashMap::new(),
            unique_id_counter: 1,
        }
    }

    pub fn add_player(&mut self, addr: SocketAddr, stream: TcpStream) -> usize {
        let id = self.unique_id_counter;
        self.unique_id_counter += 1;
    
        let mut player = Player {addr: addr, stream: stream, input_buffer: Vec::new(), output_buffer: Vec::new(), connection_status: Conn::GetName, character_name: String::new(), sex: Sex::None, race: Race::None, origin: Origin::None};
    
        // Append the greeting message to the output buffer
        let greeting_message = format!("{}\nWhat is your name?\n", GREETING);
        player.append_to_output_buffer(greeting_message);
    
        self.players.insert(id, player);
    
        id
    }

    pub fn remove_player(&mut self, id: usize) {
        self.players.remove(&id);
    }

    pub fn get_connection_status(&self, id: usize) -> Conn {
        let player = self.players.get(&id).unwrap();
        player.connection_status
    }

    pub fn is_player_online(&self, character_name: &str) -> bool {
        for player in self.players.values() {
            if player.character_name.to_lowercase() == character_name.to_lowercase() {
                return true;
            }
        }
        false
    }

    pub fn read_player_input(&mut self, id: usize) -> String {
        let player = self.players.get_mut(&id).unwrap();
        player.read_input_buffer()
    }

    pub fn send_message(&mut self, id: usize, message: String) {
        let player = self.players.get_mut(&id).unwrap();
        player.append_to_output_buffer(message + "\n");
    }

    pub fn send_global_message(&mut self, message: String) {
        for player in self.players.values_mut() {
            player.append_to_output_buffer(message.clone() + "\n");
        }
    }
}

 