use std::env::args;

use client_display::*;
use client_profile::*;

fn get_profile() -> Profile {
    match args().nth(1) {
        Some(file_path) => match Profile::load(file_path.clone()) {
            Ok(profile) => profile,
            Err(err) => {
                println!("Error whilst loading {}: {}", file_path, err);
                Profile::new(file_path)
            }
        },
        None => Profile::new("new_project".to_string()),
    }
}

fn main() {
    let profile = get_profile();
    profile.save();
    let is_toggled = false;

    ClientDisplay {
        profile,
        is_toggled,
    }
    .run_display();
}
