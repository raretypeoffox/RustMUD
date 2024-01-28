
#![allow(dead_code)]

use super::{PlayerManager, Player};
use super::db::UserDatabase;

use super::consts::constants::*;

// TODO: add reconnection logic

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
    
    // New user, set their password and add them to the db
    } else if player.connection_status == Conn::GetNewPassword {
        let password = input.trim().to_string();
    
        // Check if the user already exists
        if !database.check_user_exists(&player.character_name)? {
            // Store the new user in the users HashMap
            database.add_user(&player.character_name, &password)?;
    
            player.connection_status = Conn::GetNewSex;
            player.append_to_output_buffer("Please choose your sex [M/F/N]\n".to_string());
        } else {
            // shouldn't be reached
            player.append_to_output_buffer("Username already taken. Please choose a different one:\n".to_string());
        }

    // existing user, check their password against the db
    } else if player.connection_status == Conn::GetPassword {
        let password = input.trim().to_string();

        if database.check_password(&player.character_name, &password)? {
            player.connection_status = Conn::Playing;
            player.append_to_output_buffer("Welcome back!\n".to_string());
        } else {
            player.append_to_output_buffer("Invalid password. Please try again:\n".to_string());
        }

    } else if player.connection_status == Conn::GetNewSex {
        let input = input.trim().to_string().to_lowercase();

        fn set_sex(player: &mut Player, sex: Sex) {
            player.sex = sex;
            player.connection_status = Conn::GetNewRace;
            player.append_to_output_buffer(RACE_MSG.to_string());
        }
        
        match input.as_str() {
            "m" => set_sex(player, Sex::Male),
            "f" => set_sex(player, Sex::Female),
            "n" => set_sex(player, Sex::Neutral),
            _ => player.append_to_output_buffer("Invalid input. Please choose your sex [M/F/N]\n".to_string()),
        }

    } else if player.connection_status == Conn::GetNewRace {
        let input = input.trim().to_string().to_lowercase();

        if input.len() < 2 {
            player.append_to_output_buffer("Invalid input. Please enter at least two characters:\n".to_string());
        } else {
            
            fn set_race(player: &mut Player, race: Race) {
                player.race = race;
                player.connection_status = Conn::GetNewOrigin;
                player.append_to_output_buffer(ORIGIN_MSG.to_string());
            }
            
            match input.as_str() {
                _ if "cragkin".starts_with(&input) => set_race(player, Race::Cragkin),
                _ if "moonshade".starts_with(&input) => set_race(player, Race::Moonshade),
                _ if "etherial".starts_with(&input) => set_race(player, Race::Etherial),
                _ if "starfolk".starts_with(&input) => set_race(player, Race::Starfolk),
                _ if "frostling".starts_with(&input) => set_race(player, Race::Frostling),
                _ if "auroran".starts_with(&input) => set_race(player, Race::Auroran),
                _ => player.append_to_output_buffer("Invalid input. Please choose your race again:\n".to_string()),
            }

        }
    } else if player.connection_status == Conn::GetNewOrigin {
        let input = input.trim().to_string();

        match input.parse::<i32>() {
            Ok(number) if number >= 1 && number <= 6 => {
                fn set_origin(player: &mut Player, origin: Origin) {
                    player.origin = origin;
                    player.connection_status = Conn::ReadMotd;
                    player.append_to_output_buffer(MOTD_MSG.to_string());
                }

                match number {
                    1 => set_origin(player, Origin::WarriorOfTheForgottenLegion),
                    2 => set_origin(player, Origin::ElementalEnvoy),
                    3 => set_origin(player, Origin::SpiritualWanderer),
                    4 => set_origin(player, Origin::ShadowGuildOperative),
                    5 => set_origin(player, Origin::BorderlandSentinel),
                    6 => set_origin(player, Origin::WanderingBard),
                    _ => player.append_to_output_buffer("Invalid input. Please enter a number between 1 and 6:\n".to_string()),
                }
            },
            _ => player.append_to_output_buffer("Invalid input. Please enter a number between 1 and 6:\n".to_string()),
        }
    } else if player.connection_status == Conn::ReadMotd {
        player.connection_status = Conn::Playing;
        player.append_to_output_buffer("Welcome to the game!\n".to_string());
    }
    Ok(())
}

