# wtf

**A small helper to display cheatsheets, shortcuts and others inside your terminal.**

![Demo](assets/demo.gif)

## Requirements

- fzf
- config file (see Configuration)

## Installation

Download [latest release](https://github.com/TheAxelander/wtf/releases/latest), extract the binary and place it somewhere like in `~/.local/bin/`

## Configuration

See below example config file which is required in `~/.config/wtf/wtf.conf`

``` ini
CHEATSHEET_REPO=~/.local/share/wtf
PREVIEW_COMMAND=cat #You could also use wtf itself to render the file inside the fzf preview
```

## Usage

Without any parameter `fzf` is used to select a file which is then rendered. The content must be Markdown-based.

``` bash
wtf

>>> fzf screen is shown
```

Appending a `sheet` (simple file name) renders the file directly without any further selection

``` bash
wtf tmux

>>> tmux file will be displayed
```

By passing `--update-repo` you can pull the latest changes for your cheatsheet repo

``` bash
wtf --update-repo
```
