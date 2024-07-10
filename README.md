# THIS PROJECT IS STILL IN EARLY DEVELOPMENT
THE README IS HERE TO GIVE AN IDEA OF WHAT THE PROJECT WILL BE\
KEEP IN MIND THAT SOME FEATURES MAY NOT BE IMPLEMENTED YET\
BECAUSE OF THAT, SOME FEATURES MAY DRASTICALLY CHANGE OR BE REMOVED BEFORE THE FINAL RELEASE\
IF YOU WANT TO CONTRIBUTE TO THE PROJECT, YOU CAN OPEN A PULL REQUEST OR AN ISSUE

# Depot
Every linux user has encounter the problem of learning how a new project manager works when changing operating system\
Depot solves this problem by guessing your project manager depending on your operating system.\
When you ask Depot to install a package, Depot will start by guessing your project manager and then ask the project manager to install the package.\
In top of that, Depot will also store the package name you asked inside a file, you can then bring this file to another computer and ask Depot to install all the packages you need.

## Changelog
**v0.0.1 :** [Read changelog](./CHANGELOG.md) <!-- x-release-please-version -->

## Installation
```bash
git clone git@github.com:Rignchen/Depot.git
cd Depot
cargo build --release
mv target/release/depot /usr/local/bin
```

## Configuration
Depot will try to guess your package manager depending on your operating system, however sometimes you may want to specify it yourself, for example on Arch Linux you may want to use `yay` instead of `pacman`.\
To do so you have 2 options:
1. Set the environment variable `DEPOT_PM` to the name of your package manager
2. Add the flag `--pm <package_manager>` when running a command

## Usage
Depot has 5 commands:
- `install <package>`: Install a package
- `uninstall <package>`: Uninstall a package
- `update` <package>: Update a package, if no package is specified, update all the packages
- `list`: List all the packages you have installed
- `sync`: Compare the packages installed on the computer with the ones stored in the file and install the missing ones

## Example
In this example I will install `neovim` and `zsh` on both my computers
```bash
Depot install neovim
Depot install zsh
scp ~/.depot/packages user@computer:~/.depot/packages
ssh user@computer
Depot sync
```

## Supported package managers
- pacman
- yay
- apt
- apt-get
- pkg
- dnf
- apk

## License
This project is licensed under the Creative Commons NonCommercial-ShareAlike 4.0 International License - see the [LICENSE.md](LICENSE.md) file for details

