![Truffleyard Logo](images/truffleyard.png)

# forensic automation tool using the SANS Windows Forensic Analysis Poster (for now)

## Tutorial (Linux)

## prerequisites (if needed):
-> Download and install Rust (using rustup is recommended) on www.rust-lang.org/tools/install
-> Download and install cargo via package manager (e.g. sudo apt install cargo)
-> Download git via package manager (e.g. sudo apt install git)

## Download and Install:
1. git clone this repository in desired location
```
git clone https://github.com/sn00m4n/truffleyard.git
```
2. cd into cloned Truffleyard folder
3. install using cargo
```
cargo install --path .
```

## HOW TO USE

for now, this works only if the image is mounted in filesystem (read-only) using losetup
1. sudo losetup --find --partscan --show  --read-only /path/to/image.dd
2. sudo mount \<loopdevice\> \<mountpoint\>

tool uses the target_mountpoint as argument for now (will be changed eventually)

help:
```
truffleyard -h
```

Usage: truffleyard \[OPTIONS\] -i \<IMAGE\_PATH\> -v \<VIDPID\_PATH\> \<COMMAND\>

Commands:
  all: Analyzes everything (that's implemented so far)
  registry: Analyzes only Registry artifacts (that are implemented so far)
  event-logs: Analyzes only EventLog artifacts (that are implemented so far)
  account-usage: Analyzes Account Usage artifacts
  external-devices: Analyzes External Devices and USB usage artifacts
  system-information: Analyzes System Information artifacts
  help: Print this message or the help of the given subcommand(s)

Options:
  -i \<IMAGE\_PATH\>: path where mounted image is located
  -o \<OUTPUT\_PATH\>: output path, default is working directory \[default: .\]
  -f, --folder-name \<FOLDER\_NAME\>: name of result-folder, default is "results" \[default: results\]
  -v \<VIDPID\_PATH\>: path to file that contains vid&pid
  -h, --help: Print help
  -V, --version: Print version

