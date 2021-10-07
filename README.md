# webeep-cli

A tool to use the [WeBeep](https://webeep.polimi.it/login/index.php) platform of the Politecnico di Milano directly from the command line.

[![License](https://img.shields.io/badge/license-GPL-blue)](https://github.com/simoneorlando97/webeep-cli/blob/master/LICENSE)
[![Language](https://img.shields.io/badge/Built%20with-Rust-Purple)](https://www.rust-lang.org/)
[![Downloads](https://img.shields.io/github/downloads/simoneorlando97/webeep-cli/total?label=GitHub%20Downloads)]()

## Features
- Browse the course folders as if they were local on your pc
- Download single files, multiple files or entire folders with one simple command
- View direct links related to specific files or folders and open them with a single click directly in your browser

## Attention
The webeep display must be the default matrix display.


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
sudo mv ./webeep_linux /usr/bin/webeep
```
On Mac:
```bash
sudo mv ./webeep_macos /usr/local/bin/webeep
```
For Windows users it is recommended to run the program within powershell.
## Usage
The first time you start webeep-cli you will be asked for the credentials of the Politecnico di Milano.
```bash
webeep
```
If you change the password on the online services you can force the updating of the credentials on webeep-cli via
```bash
webeep --login
```
Once logged in, you can list your course folders via
```bash
ls
```
If you also want the respective links of files and folders you can use
```bash
ls -l
```
You can navigate through the folders using
```bash
cd number_associated_with_the_folder
```
To find out which course you are in you can use
```bash
pwd
```
You can download files via
```bash
get n_1 n_2 n_3 -d destination_path
```
where n_* are the numbers associated with the files you want to download and destination_path is the folder where you want to save these files.

If you want, instead, to download all the files in a folder you can use the command
```bash
get -all destination_path
```
To clean the console
```bash
clear
```
and finally to exit webeep-cli
```bash
exit
```