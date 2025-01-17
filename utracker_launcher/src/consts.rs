use once_cell::sync::Lazy;

#[cfg(target_os = "windows")]
pub const LAUNCHER_NAME: &str = "ArchipelagoLauncher.exe";
#[cfg(target_os = "linux")]
pub const LAUNCHER_NAME: &str = "ArchipelagoLauncher";

pub const TRACKER_ARG: &str = "Universal Tracker";

pub const PLAYERS_PROMPT: &str = "Choose players (select with space, confirm with enter)";
pub const PORT_PROMPT: &str = "Select a port";
pub const LOC_PROMPT: &str = "Archipelago location (leave empty for current directory)";

pub const CONFIG_FILE: &str = "config.toml";
pub const DEFAULT_HOST: &str = "archipelago.gg";
pub const YAML_EXT: &str = "yaml";
pub const PLAYERS_DIR: &str = "Players";

pub static HAS_ANSI: Lazy<bool> =
    Lazy::new(|| ::supports_color::on(::supports_color::Stream::Stderr).is_some_and(|f| f.has_256));
pub static THEME: Lazy<Box<dyn dialoguer::theme::Theme + 'static + Send + Sync>> =
    Lazy::new(|| {
        if *HAS_ANSI {
            Box::new(dialoguer::theme::ColorfulTheme::default())
        } else {
            Box::new(dialoguer::theme::SimpleTheme)
        }
    });
