
use std::io;
use super::PlayerManager;

use super::consts::constants::*;

pub fn process_player_login(player_manager: &mut PlayerManager, player_id: usize,) -> io::Result<()> {

    let player = player_manager.players.get_mut(&player_id).unwrap();
    let input = player.read_input_buffer();

    if player.connection_status == Conn::GetName {


    }


    Ok(())
}