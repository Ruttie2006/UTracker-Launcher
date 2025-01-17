#![warn(clippy::print_stdout, clippy::print_stderr)]

use args::Args;
use std::process::Command;

pub(crate) mod args;
pub(crate) mod consts;
pub(crate) mod error;
pub(crate) mod macros;
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
        info!("Running user {}", user);

        let mut cmd = Command::new(program_loc.clone());
        cmd.arg(consts::TRACKER_ARG)
            .arg("--")
            .arg("--name")
            .arg(user)
            .arg("--connect")
            .arg(format!("{}:{}", host, port))
            .current_dir(location.clone());

        if let Err(err) = cmd.spawn() {
            warn!("Failed to run user: {}", err);
        }
    }
}
