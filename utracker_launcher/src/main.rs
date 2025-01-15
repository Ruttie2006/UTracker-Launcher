use std::process::Command;

use args::Args;

pub(crate) mod args;
pub(crate) mod consts;
pub(crate) mod error;
pub(crate) mod player;

pub type Result<T = ()> = std::result::Result<T, error::Error>;

fn main() {
    let Args {
        location,
        port,
        host,
        players,
    } = match Args::get() {
        Ok(val) => val,
        Err(err) => err.consume(),
    };

    let program_loc = location.join(consts::LAUNCHER_NAME);

    for user in players {
        println!("Running user {}", user);

        let mut cmd = Command::new(program_loc.clone());
        cmd.arg(consts::TRACKER_ARG)
            .arg("--")
            .arg("--name")
            .arg(user)
            .arg("--connect")
            .arg(format!("{}:{}", host, port))
            .current_dir(location.clone());

        if let Err(err) = cmd.spawn() {
            eprintln!("Failed to run user: {}", err);
        }
    }
}
