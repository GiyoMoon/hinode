use astrolabe::{DateTime, DateUtilities, Offset, OffsetUtilities};
use clap::Parser;
use std::{process::Command, thread, time::Duration};
use sunrise::sunrise_sunset;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct HinodeArgs {
    /// Latitude of the location
    #[arg(long)]
    latitude: f64,
    /// Longitude of the location
    #[arg(long)]
    longitude: f64,
    /// Command to run when switching to light mode
    #[arg(long)]
    light_mode_cmd: String,
    /// Command to run when switching to dark mode
    #[arg(long)]
    dark_mode_cmd: String,
    /// Whether to log debug information
    #[arg(long, default_value = "false")]
    debug: bool,
}

fn main() {
    let args = HinodeArgs::parse();
    if args.debug {
        println!("Starting hinode");
        println!("====================");
        println!("Latitude: {}", args.latitude);
        println!("Longitude: {}", args.longitude);
        println!("Light mode command: {}", args.light_mode_cmd);
        println!("Dark mode command: {}", args.dark_mode_cmd);
        println!("====================");
    }

    sync_current_mode(&args);

    loop {
        let now = DateTime::now_local();
        let (sunrise_today, sunset_today) = sunrise_sunset(
            args.latitude,
            args.longitude,
            now.year(),
            now.month(),
            now.day(),
        );
        let next_event = if now.timestamp() < sunrise_today {
            sunrise_today
        } else if now.timestamp() < sunset_today {
            sunset_today
        } else {
            let tomorrow = now.add_days(1);
            sunrise_sunset(
                args.latitude,
                args.longitude,
                tomorrow.year(),
                tomorrow.month(),
                tomorrow.day(),
            )
            .0
        };
        let target_mode = if now.timestamp() < sunrise_today || now.timestamp() > sunset_today {
            "light"
        } else {
            "dark"
        };
        let offset_secs = next_event - now.timestamp();

        if args.debug {
            println!(
                "Waiting until {} to switch to {} mode",
                DateTime::from_timestamp(next_event).set_offset(Offset::Local),
                target_mode
            );
        }

        thread::sleep(Duration::from_secs(offset_secs as u64));
        run_theme_cmd(target_mode, &args);
    }
}

fn sync_current_mode(args: &HinodeArgs) {
    if args.debug {
        println!("====Syncing current mode====");
    }

    let appearance_output = Command::new("osascript")
        .args([
            "-l",
            "JavaScript",
            "-e",
            "Application('System Events').appearancePreferences.darkMode()",
        ])
        .output()
        .expect("Failed to get current system appearance mode");

    let is_dark_mode = appearance_output.stdout.starts_with(b"true");

    let now = DateTime::now_local();
    let (sunrise_today, sunset_today) = sunrise_sunset(
        args.latitude,
        args.longitude,
        now.year(),
        now.month(),
        now.day(),
    );

    let current_expected_mode = if now.timestamp() < sunrise_today || now.timestamp() > sunset_today
    {
        "dark"
    } else {
        "light"
    };

    if args.debug {
        println!("Current:  {}", if is_dark_mode { "dark" } else { "light" });
        println!("Expected: {}", current_expected_mode);
    }

    if (is_dark_mode && current_expected_mode == "light")
        || (!is_dark_mode && current_expected_mode == "dark")
    {
        run_theme_cmd(current_expected_mode, args);
    }
    if args.debug {
        println!("====================");
    }
}

fn run_theme_cmd(mode: &str, args: &HinodeArgs) {
    let cmd = if mode == "light" {
        args.light_mode_cmd.clone()
    } else {
        args.dark_mode_cmd.clone()
    };
    let arguments = cmd.split_whitespace().skip(1).collect::<Vec<&str>>();
    let program = cmd.split_whitespace().next().unwrap();

    Command::new(program)
        .args(arguments)
        .output()
        .unwrap_or_else(|_| panic!("Failed to run command: {}", cmd));

    if args.debug {
        println!("Switched to {} mode", mode);
    }
}
