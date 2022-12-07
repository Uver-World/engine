use client_display::*;
use client_profile::*;

fn main() {
    let profile = Profile::load("new_project".to_string());

    match profile {
        Ok(profile) => {
            ClientDisplay { profile }.run_display();
        }
        Err(err) => {
            println!("Error whilst loading new_project: {}", err);
            let profile = Profile::default();
            profile.save();
            ClientDisplay { profile }.run_display();
        }
    }
}
