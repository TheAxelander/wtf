import argparse
import subprocess
from dotenv import dotenv_values
from rich.console import Console
from rich.table import Table
from pathlib import Path
from wtf.__about__ import __version__

console = Console()

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--version", "-v", action="version", version=f"wtf {__version__}")
    parser.add_argument('sheet', nargs='?', type=str, help="Specific cheatsheet which should be printed")

    try:
        config = get_config()
        directory = Path(config['CHEATSHEET_REPO']).expanduser()
    except Exception as e:
        console.print(f"[bold red]Error reading config: {e}[/bold red]")
        exit(1)

    args = parser.parse_args()
    if args.sheet:
        render_file(directory / args.sheet)
    else:
        select_file_via_fzf(directory)

def get_config():
    """
    Reads configuration from the wtf.conf file. Returns a dictionary of configuration values.
    """
    config_path = Path.home() / '.config' / 'wtf' / 'wtf.conf'
    if not config_path.exists():
        raise FileNotFoundError(f"Config file not found: {config_path}")

    return dotenv_values(config_path)


def select_file_via_fzf(directory):
    """
    Runs fzf to allow the user to select a file from the given directory.
    
    :param directory: Path object representing the directory to list files from.
    """
    if not directory.exists():
        console.print(f"[bold red]Directory {directory} does not exist![/bold red]")
        exit(1)

    # Run fzf to list files in from directory
    try:
        try:
            config = get_config()
            preview_command = config.get('PREVIEW_COMMAND', 'cat')
        except Exception as e:
            console.print(f"[bold red]Error reading config: {e}[/bold red]")
            exit(1)
            
        selected_file = subprocess.check_output(
            # Run fzf and show a preview of the file content
            ['fzf', '--preview', f'{preview_command} {{}}', '--preview-window', 'top:75%'],
            cwd=str(directory),  # working directory
            text=True
        ).strip()

        if selected_file:
            render_file(directory / selected_file)

    except subprocess.CalledProcessError as e:
        # Exit code 130 means user pressed ESC, handle silently
        if e.returncode == 130:
            return
        console.print(f"[bold red]Error with fzf command: {e}[/bold red]")


def render_file(file_path):
    """
    Renders the content of the given file as a table in the terminal.
    
    :param file_path: Path object representing the file to be rendered
    """
    if not file_path.exists():
        console.print(f"[red]File not found: {file_path}[/red]")
        return

    try:
        config = get_config()
        delimiter = config.get('TABLE_DELIMITER', ',')
    except Exception as e:
        console.print(f"[bold red]Error reading config: {e}[/bold red]")
        exit(1)

    with open(file_path, 'r') as f:
        lines = [line.strip() for line in f if line.strip()]

    if not lines:
        console.print("[yellow]File is empty.[/yellow]")
        return

    headers = lines[0].split(delimiter)
    table = Table(show_header=True, header_style="bold cyan", show_lines=True)

    for header in headers:
        table.add_column(header.strip())
    table.columns[0].no_wrap=True

    for line in lines[1:]:
        row = [col.strip() for col in line.split(delimiter)]
        table.add_row(*row)

    console.print(table)


if __name__ == '__main__':
    main()
