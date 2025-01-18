#![deny(
    clippy::print_stdout,
    clippy::print_stderr,
    clippy::unwrap_used,
    clippy::expect_used,
    unsafe_code
)]

use args::Args;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::process::Command;

pub(crate) mod args;
pub(crate) mod consts;
pub(crate) mod error;
pub(crate) mod macros;

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

    if !program_loc.exists() {
        crate::error::Error::Validation(crate::error::ValidationError::NoArchipelago).consume();
    }

    let url = format!("{}:{}", host, port);

    players.into_par_iter().for_each(|user| {
        info!("Running user {}", user);

        if let Err(err) = match Command::new(&program_loc)
            .arg(consts::TRACKER_ARG)
            .arg("--")
            .arg("--name")
            .arg(&user)
            .arg("--connect")
            .arg(&url)
            .current_dir(&location)
            .spawn()
            .map(|mut f| f.wait())
        {
            Ok(inner) => inner, // required due to `result_flattening` not being stable yet
            Err(err) => Err(err),
        } {
            warn!("Failed to run user '{}': {}", user, err);
        }
    });
}
