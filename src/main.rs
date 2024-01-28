// main.rs
#![allow(dead_code)]

use std::net::TcpListener;
use std::io::{self, Read, Write};


mod handler; 
mod consts;
mod login;
mod db;
mod player;

use handler::process_player_input;
use login::process_player_login; 
use consts::constants::{Conn, Sex, Race, Origin}; 
use player::{Player, PlayerManager};

fn main() -> io::Result<()> {
    // Bind the server to a local port
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    listener.set_nonblocking(true)?;

    let mut data_buf = [0; 512];

    let mut player_manager = PlayerManager::new();

    loop {
        // Accept new connections and add them to the client list
        match listener.accept() {
            Ok((stream, addr)) => {
                println!("New client: {}", addr);
                stream.set_nonblocking(true)?;
                player_manager.add_player(addr, stream);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // No incoming connection yet
            }
            Err(e) => panic!("encountered IO error: {}", e),
        }

        let mut players_input_to_process = Vec::new();
        let mut disconnected_players = Vec::new();

        // Read incoming data from clients
        for (id, player) in player_manager.players.iter_mut() {
            match player.stream.read(&mut data_buf) {
                Ok(0) => {
                    // Connection was closed
                    println!("Client {} disconnected", player.addr);
                    disconnected_players.push(*id);
                }
                Ok(len) => {
                    //println!("Received data from {}: {:?}", player.addr, &data_buf[..len]);
                    player.append_to_input_buffer(&data_buf[..len]);
                    players_input_to_process.push(*id);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // No data received yet
                }
                Err(e) => {
                    println!("Failed to receive data from {}: {}", player.addr, e);
                }
            }
        }

        // Handle disconnections
        for id in disconnected_players {
            player_manager.remove_player(id);
        }

        // Process input from clients
        for id in players_input_to_process {
            if player_manager.get_connection_status(id) == Conn::Playing {
                match process_player_input(&mut player_manager, id) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("Failed to process input from player {}: {}", id, e);
                    }
                }
            } else {
                match process_player_login(&mut player_manager, id) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("Failed to process input from player {}: {}", id, e);
                    }
                }
            }
        }
        
        // Process output to clients
        for player in player_manager.players.values_mut() {
            if !player.output_buffer.is_empty() {

                // if logged in, append prompt
                if player.connection_status == Conn::Playing {
                    player.append_to_output_buffer("\n<HP Ma XP>\n".to_string());
                }

                match player.stream.write_all(&player.output_buffer) {
                    Ok(_) => {
                        player.output_buffer.clear();
                        if let Err(e) = player.stream.flush() {
                            println!("Failed to flush data to {}: {}", player.addr, e);
                        }
                    }
                    Err(e) => {
                        println!("Failed to send data to {}: {}", player.addr, e);
                    }
                }
            }
        }


        // Update game state
    }
}
