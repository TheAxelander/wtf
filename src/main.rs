use clap::Parser;
use std::collections::HashMap;
use std::{env, fs};
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use termimad::MadSkin;

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

    let repo_path = config
        .get("CHEATSHEET_REPO")
        .ok_or("CHEATSHEET_REPO not defined")?;

    let path = PathBuf::from_str(repo_path)
        .map_err(|_| "CHEATSHEET_REPO is not a valid path")?;

    if args.update_repo {
        update_repo(&path)?;
        return Ok(());
    }

    if let Some(sheet) = args.sheet {
        render_file(&path.join(sheet))?;
        return Ok(());
    }

    select_file_via_fzf(&path)?;

    Ok(())
}

/// Reads configuration from the wtf.conf file. Returns a HashMap of configuration values.
fn get_config() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let config_path = dirs::home_dir()
        .ok_or("Could not determine home directory")?
        .join(".config/wtf/wtf.conf");

    dotenvy::from_path_override(&config_path)
        .map_err(|e| format!("Config file not found: {e}"))?;

    let mut result: HashMap<String, String> = Default::default();

    // CHEATSHEET_REPO
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

    // PREVIEW_COMMAND
    let key = "PREVIEW_COMMAND".to_string();
    if let Ok(val) = env::var(&key) {
        result.insert(key, val);
    }

    Ok(result)
}

/// Updates the cheatsheet repository from the passed directory by running `git pull --rebase`.
fn update_repo(directory: &PathBuf) -> Result<(), Box<dyn Error>> {
    let status = Command::new("git")
        .args(["pull", "--rebase"])
        .current_dir(directory)
        .status()
        .map_err(|e| format!("Unable to run git pull: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Git pull failed with exit code {}", status
            .code()
            .unwrap_or(-1))
            .into())
    }
}

/// Runs `fzf` to allow the user to select a file from the given directory.
fn select_file_via_fzf(directory: &PathBuf) -> Result<(), Box<dyn Error>> {
    directory.try_exists()?;

    let config = get_config()?;

    let preview_command = config.get("PREVIEW_COMMAND")
        .map(String::as_str)
        .unwrap_or("cat");
        
    let output = Command::new("fzf")
        .args(["--preview", &format!("{} {{}}", preview_command), "--preview-window", "top:75%"])
        .current_dir(directory)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::piped())
        .output()
        .map_err(|e| format!("Unable to run fzf: {e}"))?;

    if !output.status.success() {
        // Exit code 130 means user pressed ESC, handle silently
        if output.status.code() == Some(130) {
            return Ok(());
        }
        let status_code = output.status.code().unwrap_or(-1);
        return Err(format!("fzf failed with exit code {}", status_code).into());
    }

    let selected_file = String::from_utf8(output.stdout)?
        .trim()
        .to_string();

    render_file(&directory.join(selected_file))?;

    Ok(())
}

/// Renders the content of the given Markdown file in the terminal.
fn render_file(file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut path = file_path.clone();
    if !matches!(path.try_exists(), Ok(true)) {
        path.set_extension("md");
        if !matches!(path.try_exists(), Ok(true)) {
            return Err(format!("File not found: {}", path.display()).into());
        }
    }

    let content = fs::read_to_string(&path)?;
    let skin = MadSkin::default();

    println!("{}", skin.term_text(&content));

    Ok(())
}