# webeep-cli

A tool to use the [WeBeep](https://webeep.polimi.it/login/index.php) platform of the Politecnico di Milano directly from the command line.

[![License](https://img.shields.io/badge/license-GPL-blue)](https://github.com/simoneorlando97/webeep-cli/blob/master/LICENSE)
[![Language](https://img.shields.io/badge/Built%20with-Rust-Purple)](https://www.rust-lang.org/)
[![Downloads](https://img.shields.io/github/downloads/simoneorlando97/webeep-cli/total?label=GitHub%20Downloads)]()

## Features
- Browse the course folders as if they were local on your pc
- Download single files, multiple files or entire folders with one simple command
- View direct links related to specific files or folders and open them with a single click directly in your browser

## Installation
Download the release corresponding to your operating system from the panel on the right or directly from your terminal.


For Linux:
```bash
wget https://github.com/simoneorlando97/webeep-cli/releases/download/v1.0/webeep_linux
```
For Mac:
```bash
wget https://github.com/simoneorlando97/webeep-cli/releases/download/v1.0/webeep_macos
```
For Windows:
```bash
wget https://github.com/simoneorlando97/webeep-cli/releases/download/v1.0/webeep_win.exe
```
Once downloaded you need to give it execute permissions.

On Linux:
```bash
chmod +x ./webeep_linux
```
On Mac:
```bash
chmod +x ./webeep_linux
```
Finally, to be able to call webeep at any time from your terminal.

On Linux:
```bash
sudo mv ./webeep_linux $HOME/bin/webeep
```
On Mac:
```bash
sudo mv ./webeep_macos /usr/local/bin/webeep
```
For Windows users it is recommended to run the program within powershell.
## Usage
Test