
#![allow(dead_code)]

use super::PlayerManager;
use super::db::UserDatabase;

use super::consts::constants::*;

pub fn process_player_login(player_manager: &mut PlayerManager, player_id: usize,) -> Result<(), Box<dyn std::error::Error>> {

    let player = player_manager.players.get_mut(&player_id).unwrap();
    let input = player.read_input_buffer();
    let database = UserDatabase::new()?;
    database.create_users_table()?;

    if player.connection_status == Conn::GetName {
        player.character_name = input.trim().to_string();

        if database.check_user_exists(&player.character_name)? {
            // The user exists, ask for password
            player.connection_status = Conn::GetPassword;
            player.append_to_output_buffer("Existing character.".to_string());
            player.append_to_output_buffer("Please enter your password:\n".to_string());
        } else {
            // The user doesn't exist, ask for a new password
            player.connection_status = Conn::GetNewPassword;
            player.append_to_output_buffer("Please enter a new password:\n".to_string());
        }
    } else if player.connection_status == Conn::GetNewPassword {
        let password = input.trim().to_string();
    
        // Check if the user already exists
        if !database.check_user_exists(&player.character_name)? {
            // Store the new user in the users HashMap
            database.add_user(&player.character_name, &password)?;
    
            player.connection_status = Conn::Playing;
            player.append_to_output_buffer("Welcome!\n".to_string());
        } else {
            player.append_to_output_buffer("Username already taken. Please choose a different one:\n".to_string());
        }
    } else if player.connection_status == Conn::GetPassword {
            let password = input.trim().to_string();
    
            if database.check_password(&player.character_name, &password)? {
                player.connection_status = Conn::Playing;
                player.append_to_output_buffer("Welcome back!\n".to_string());
            } else {
                player.append_to_output_buffer("Invalid password. Please try again:\n".to_string());
            }
        }


    Ok(())
}


// pub fn process_player_login(player_manager: &mut PlayerManager, player_id: usize,) -> io::Result<()> {
//     let player = player_manager.players.get_mut(&player_id).unwrap();
//     let input = player.read_input_buffer();

//     if player.connection_status == Conn::GetName {
//         let username = input.trim().to_string();

//         if player_manager.users.contains_key(&username) {
//             // The user exists, ask for password
//             player.connection_status = Conn::GetPassword;
//             player.append_to_output_buffer("Please enter your password:\n");
//         } else {
//             // The user doesn't exist, ask for a new password
//             player.connection_status = Conn::GetNewPassword;
//             player.append_to_output_buffer("Please enter a new password:\n");
//         }
//     } else if player.connection_status == Conn::GetNewPassword {
//         let password = input.trim().to_string();
//         let hashed_password = hash(&password, DEFAULT_COST)?;

//         // Store the new user in the users HashMap
//         player_manager.users.insert(player.username.clone(), hashed_password);

//         player.connection_status = Conn::Playing;
//         player.append_to_output_buffer("Welcome!\n");
//     }

//     Ok(())
// }

// if database.users.contains_key(&username) {
//     // The user exists
// } else {
//     // The user doesn't exist
// }