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
    let args = Args::parse();

    let config = get_config();
    match config {
        Ok(config) => {
            if args.update_repo {
                let err: String;
                if let Some(val) = config.get("CHEATSHEET_REPO") {
                    let path = PathBuf::from_str(val);
                    match path {
                        Ok(path) => {
                            let update_repo_result = update_repo(&path);
                            match update_repo_result {
                                Ok(_) => return,
                                Err(e) => err = format!("{e}"),
                            }
                        }
                        Err(_) => err = "CHEATSHEET_REPO is not a valid path".to_string()
                    }

                } else {
                    err = "CHEATSHEET_REPO not defined".to_string();

                }
                println!("{err}");
                return;
            }

            if let Some(sheet) = args.sheet {
                render_file();
                return;
            }
        }
        Err(e) => {
            println!("Error reading config {e}");
            return;
        }
    }
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

    if !matches!(dotenvy::from_path_override(&config_path), Ok(())) {
        return Err(format!("Config file not found: {}", &config_path.display()).into());
    }

    let mut result: HashMap<String, String> = Default::default();

    let key = "CHEATSHEET_REPO".to_string();
    if let Ok(val) = env::var(&key) {
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
    } else {
        return Err("Cheatsheet repo not defined in config".to_string().into());
    }

    Ok(result)
}

fn update_repo(directory: &PathBuf) -> Result<(), String> {
    let status = Command::new("git")
        .args(["pull", "--rebase"])
        .current_dir(directory)
        .status();

    match status {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Unable to run git pull: {e}"))
    }
}

fn render_file() {
    // TODO Implement file output
}