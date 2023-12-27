use std::path::Path;
use clap::{Arg, ArgMatches, Command};

use client_display::*;
use client_profile::*;

fn get_profile(matches: &ArgMatches) -> Result<Profile, String> {
    let file_path: Option<&String> = matches.get_one("profile");

    if let Some(file_path) = file_path {
        return match Profile::load(file_path.into()) {
            Ok(profile) => Ok(profile),
            Err(err) => {
                eprintln!("Error whilst loading {}: {}", file_path, err);
                Ok(Profile::new(file_path.into()))
            }
        }
    }
    Ok(Profile::new("new_project".to_string()))
}

fn get_env(matches: &ArgMatches) -> Result<String, String> {
    let default_env = ".env".to_string();
    let env_file: Option<&String> = matches.get_one("env");

    if let Some(env_file) = env_file {
        let env_path = Path::new(env_file);
        if !env_path.exists() {
            return Err(format!("environment file: '{env_file}' does not exist"));
        }
        return Ok(env_path.to_str().unwrap().into());
    }
    Ok(default_env)
}

fn get_settings() -> Result<Settings, String> {
    let app = Command::new("Engine settings").args(
        [Arg::new("env")
            .help("Export the .env file to environment variables before run")
            .short('e'),
        Arg::new("offline")
            .help("Runs the app in offline mode.")
            .short('o')
            .action(clap::ArgAction::SetTrue),
        Arg::new("no_telemetry")
            .help("Disables the telemetry.")
            .short('t')
            .action(clap::ArgAction::SetFalse),
        Arg::new("profile")
            .help("Sets the profile")
            .index(1)],
    );

    let matches = app.get_matches();
    let env_file = get_env(&matches)?;

    dotenv::from_filename(env_file).ok();
    let profile = get_profile(&matches)?;
    profile.save();

    Ok(Settings {
        profile,
        api_settings: ApiSettings::from_env(),
        telemetry_settings: TelemetrySettings::from_env(),
        is_offline: matches.get_flag("offline"),
        has_telemetry: matches.get_flag("no_telemetry"),
    })
}

fn main() {
    let settings = match get_settings() {
        Ok(settings) => settings,
        Err(error) => panic!("An error occurred whilst starting the app: [{}]", error)
    };

    eprintln!("Offline mode = {}", settings.is_offline);
    eprintln!("Telemetry mode = {}", settings.has_telemetry);

    ClientDisplay {
        settings,
        is_toggled: true,
    }
    .run_display();
}
