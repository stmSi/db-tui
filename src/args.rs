use anyhow::{anyhow, Result};
use clap::{crate_authors, crate_description, crate_name, Arg, Command as ClapApp};
use simplelog::{Config, LevelFilter, WriteLogger};
use std::{
    env,
    fs::{self, File},
    path::PathBuf,
};

pub struct CliArgs {
    pub theme: PathBuf,
}

pub fn process_cmdline() -> Result<CliArgs> {
    let app = app();

    let arg_matches = app.get_matches();

    if arg_matches.get_flag("logging") {
        setup_logging()?;
    }

    let workdir = arg_matches.get_one::<String>("workdir").map(PathBuf::from);
    let gitdir = arg_matches
        .get_one::<String>("directory")
        .map_or_else(|| PathBuf::from("."), PathBuf::from);

    let arg_theme = arg_matches
        .get_one::<String>("theme")
        .map_or_else(|| PathBuf::from("theme.ron"), PathBuf::from);

    let theme = get_app_config_path()?.join(arg_theme);

    Ok(CliArgs { theme })
}

fn app() -> ClapApp {
    ClapApp::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("logging")
                .help("Stores logging output into a cache directory")
                .short('l')
                .long("logging")
                .num_args(0),
        )
}

fn setup_logging() -> Result<()> {
    let mut path = get_app_cache_path()?;
    path.push("db-manager.log");

    println!("Logging enabled. log written to: {path:?}");

    WriteLogger::init(LevelFilter::Trace, Config::default(), File::create(path)?)?;

    Ok(())
}

fn get_app_cache_path() -> Result<PathBuf> {
    let mut path = dirs::cache_dir().ok_or_else(|| anyhow!("failed to find os cache dir."))?;

    path.push("gitui");
    fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn get_app_config_path() -> Result<PathBuf> {
    let mut path = if cfg!(target_os = "macos") {
        dirs::home_dir().map(|h| h.join(".config"))
    } else {
        dirs::config_dir()
    }
    .ok_or_else(|| anyhow!("failed to find os config dir."))?;

    path.push("db-manager");
    fs::create_dir_all(&path)?;
    Ok(path)
}

#[test]
fn verify_app() {
    app().debug_assert();
}
