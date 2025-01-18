use std::{
    env::current_dir,
    fs,
    num::NonZeroU16,
    path::{Path, PathBuf},
};

use dialoguer::{Input, MultiSelect};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Deserialize;
use yaml_rust2::{Yaml, YamlLoader};

use crate::{
    consts,
    error::{Error, ValidationError},
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

        match players.len() {
            0 => Err(Error::Validation(ValidationError::NoPlayers)),
            1 => Ok(players),
            _ => loop {
                let res = MultiSelect::with_theme(&**consts::THEME)
                    .with_prompt(consts::PLAYERS_PROMPT)
                    .items(&players)
                    .interact()?;

                if res.is_empty() {
                    warn!("You need to select at least one player, please try again!");
                    continue;
                }

                return Ok(res.into_iter().map(|i| players[i].clone()).collect());
            },
        }
    }

    fn find_players(path: impl AsRef<Path>) -> crate::Result<Vec<String>> {
        if !path.as_ref().exists() {
            return Err(Error::Validation(ValidationError::NoPlayerDir));
        }

        let files: Vec<_> = fs::read_dir(path)?
            .filter_map(|f| {
                if let Ok(val) = f {
                    if val.file_type().is_ok_and(|f| f.is_file())
                        && val
                            .path()
                            .extension()
                            .is_some_and(|f| f == consts::YAML_EXT)
                    {
                        Some(val.path())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        let found: Vec<_> = files
            .into_par_iter()
            .filter_map(|path| {
                let str = fs::read_to_string(&path)
                    .map_err(|err| {
                        warn!(
                            "Failed to read the file '{}': {}",
                            path.to_string_lossy(),
                            err
                        );
                    })
                    .ok()?;

                let yaml = YamlLoader::load_from_str(
                    str.trim_start_matches(consts::BOM).trim_start(), // remove BOM
                )
                .map_err(|err| {
                    warn!(
                        "Failed to parse the file '{}': {}",
                        path.to_string_lossy(),
                        err
                    );
                })
                .ok()?;

                Some(yaml.into_par_iter().filter_map(move |doc| {
                    doc.as_hash()
                        .and_then(|f| f.get(&Yaml::String(String::from("name"))))
                        .and_then(|f| f.as_str().map(|f| f.to_owned()))
                        .or_else(|| {
                            warn!(
                                "No 'name' string field in player file {}",
                                path.to_string_lossy()
                            );
                            None
                        })
                }))
            })
            .flatten()
            .collect();

        Ok(found)
    }
}
