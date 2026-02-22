use clap::Parser;
use std::collections::HashMap;
use std::env;

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
    if let Err(e) = config {
        println!("Error reading config {e}");
        return;
    }

    if args.update_repo {
        update_repo();
        return;
    }

    if let Some(sheet) = args.sheet {
        render_file();
        return;
    }
}

fn get_config() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
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
        result.insert(key.to_string(), val);
    } else {
        return Err("Cheatsheet repo not defined in config".to_string().into());
    }

    Ok(result)
}

fn update_repo() {
    // TODO Implement Repo pull
}

fn render_file() {
    // TODO Implement file output
}