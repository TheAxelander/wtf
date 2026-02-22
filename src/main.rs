use clap::Parser;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

#[derive(Parser)]
#[command(name = "wtf")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A small helper to display cheatsheets, shortcuts and others inside your terminal.")]
struct Args {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    update_repo: bool,

    #[arg(value_name = "SHEET")]
    sheet: Option<String>,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let config = get_config()?;

    if args.update_repo {
        let repo_path = config
            .get("CHEATSHEET_REPO")
            .ok_or("CHEATSHEET_REPO not defined")?;

        let path = PathBuf::from_str(repo_path)
            .map_err(|_| "CHEATSHEET_REPO is not a valid path")?;

        update_repo(&path)?;
        return Ok(());
    }

    if let Some(_sheet) = args.sheet {
        render_file();
    }

    Ok(())
}

fn get_config() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let config_path = dirs::home_dir()
        .ok_or("Could not determine home directory")?
        .join(".config/wtf/wtf.conf");

    /*
    if !matches!(config_path.try_exists(), Ok(true)) {
        return Err(format!("Config file not found: {}", config_path.display()).into());
    }
    */

    dotenvy::from_path_override(&config_path)
        .map_err(|e| format!("Config file not found: {e}"))?;

    let mut result: HashMap<String, String> = Default::default();

    let key = "CHEATSHEET_REPO".to_string();
    let val = env::var(&key)
        .map_err(|_| "Cheatsheet repo not defined in config")?;

    // Expand ~ to home directory
    let expanded = if val.starts_with("~/") {
        dirs::home_dir()
            .ok_or("Could not determine home directory")?
            .join(&val[2..])
            .to_string_lossy()
            .to_string()
    } else {
        val
    };

    result.insert(key, expanded);

    Ok(result)
}

fn update_repo(directory: &PathBuf) -> Result<(), String> {
    let status = Command::new("git")
        .args(["pull", "--rebase"])
        .current_dir(directory)
        .status()
        .map_err(|e| format!("Unable to run git pull: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Git pull failed with exit code {}", status.code().unwrap_or(-1)))
    }
}

fn render_file() {
    // TODO Implement file output
}