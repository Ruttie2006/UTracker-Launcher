use std::{
    env::current_dir,
    fs::{self, OpenOptions},
    num::NonZeroU16,
    path::{Path, PathBuf},
};

use dialoguer::{Input, MultiSelect};
use serde::Deserialize;

use crate::{
    consts,
    error::{Error, ValidationError},
    player::PlayerFile,
    warn,
};

pub struct Args {
    pub location: PathBuf,
    pub port: NonZeroU16,
    pub host: String,
    pub players: Vec<String>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct ConfigArgs {
    #[serde(default)]
    location: Option<PathBuf>,
    #[serde(default)]
    port: Option<NonZeroU16>,
    #[serde(default)]
    host: Option<String>,
}

impl Args {
    pub fn get() -> crate::Result<Self> {
        let mut location = None;
        let mut port = None;
        let mut host = None;

        if fs::exists(consts::CONFIG_FILE)? {
            let val: ConfigArgs = toml::from_str(&fs::read_to_string(consts::CONFIG_FILE)?)?;
            location = val.location;
            port = val.port;
            host = val.host;
        }

        let loc = match location {
            Some(val) => val,
            None => Self::prompt_location()?.into(),
        };

        Ok(Self {
            location: loc.clone(),
            port: match port {
                Some(val) => val,
                None => Self::prompt_port()?,
            },
            host: host.unwrap_or_else(|| String::from(consts::DEFAULT_HOST)),
            players: Self::prompt_users(loc.join(consts::PLAYERS_DIR))?,
        })
    }

    fn prompt_location() -> crate::Result<String> {
        Ok(Input::with_theme(&**consts::THEME)
            .default(current_dir()?.to_string_lossy().to_string())
            .with_prompt(consts::LOC_PROMPT)
            .interact_text()?)
    }

    fn prompt_port() -> crate::Result<NonZeroU16> {
        Ok(Input::with_theme(&**consts::THEME)
            .allow_empty(false)
            .with_prompt(consts::PORT_PROMPT)
            .interact_text()?)
    }

    fn prompt_users(path: impl AsRef<Path>) -> crate::Result<Vec<String>> {
        let players = Self::find_players(path)?;

        if players.is_empty() {
            return Err(Error::Validation(ValidationError::NoPlayers));
        }

        if players.len() == 1 {
            return Ok(players.into_iter().map(|f| f.name).collect());
        }

        loop {
            let mut diag =
                MultiSelect::with_theme(&**consts::THEME).with_prompt(consts::PLAYERS_PROMPT);

            for i in &players {
                diag = diag.item(&i.name);
            }

            let res = diag.interact()?;

            if res.is_empty() {
                warn!("You need to select at least one player, please try again!");
                continue;
            }

            let mut vec = Vec::with_capacity(res.len());
            for i in res {
                vec.push(players[i].name.clone());
            }

            return Ok(vec);
        }
    }

    fn find_players(path: impl AsRef<Path>) -> crate::Result<Vec<PlayerFile>> {
        let mut found: Vec<PlayerFile> = Vec::new();

        if !std::fs::exists(&path)? {
            return Err(Error::Validation(ValidationError::NoPlayerDir));
        }

        for i in std::fs::read_dir(path)? {
            let i = match i {
                Ok(val)
                    if val.file_type().is_ok_and(|f| f.is_file())
                        && val
                            .path()
                            .extension()
                            .is_some_and(|f| f == consts::YAML_EXT) =>
                {
                    val
                }
                _ => continue,
            };

            let read = match OpenOptions::new().read(true).open(i.path()) {
                Ok(val) => val,
                Err(err) => {
                    warn!(
                        "Failed to open the file '{}': {}",
                        i.path().to_string_lossy(),
                        err
                    );
                    continue;
                }
            };
            let val = match serde_yml::from_reader(read) {
                Ok(val) => val,
                Err(err) => {
                    warn!(
                        "Failed to parse the file '{}': {}",
                        i.path().to_string_lossy(),
                        err
                    );
                    continue;
                }
            };

            found.push(val);
        }

        Ok(found)
    }
}
