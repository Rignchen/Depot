# THIS PROJECT IS STILL IN EARLY DEVELOPMENT
THE README IS HERE TO GIVE AN IDEA OF WHAT THE PROJECT WILL BE.\
KEEP IN MIND THAT SOME FEATURES MAY NOT BE IMPLEMENTED YET.\
BECAUSE OF THAT, SOME FEATURES MAY DRASTICALLY CHANGE OR BE REMOVED BEFORE THE FINAL RELEASE.\
IF YOU WANT TO CONTRIBUTE TO THE PROJECT, YOU CAN OPEN A PULL REQUEST OR AN ISSUE.

# Depot
Every Linux user has encountered the problem of learning how a new project manager works when changing operating system.\
Depot solves this problem by guessing your package manager based on your operating system.\
When you ask Depot to install a package, Depot will start by guessing your package manager and then instruct it to install the package.\
In top of that, Depot will also store the package name you requested inside a file. You can then bring this file to another computer and ask Depot to install all the packages you need.
Our inspiration for this is [Stow](https://www.gnu.org/software/stow/) a symlink farm manager. You can use Depot in combination with Stow to store your
package list in the same place as your dotfiles

## Installation
```bash
git clone git@github.com:Rignchen/Depot.git
cd Depot
cargo build --release
mv target/release/depot /usr/local/bin
```

## Configuration
Depot will try to guess your package manager based on your operating system. However, sometimes you may want to specify it yourself. For example on Arch (btw) you may want to use `yay` instead of `pacman`.\
To do so you have 2 options:
1. Set the environment variable `DEPOT_PM` to the name of your package manager
2. Add the flag `--pm <package_manager>` when running a command

## Usage
Depot has 5 commands:
- `install <package>`: Install a package.
- `uninstall <package>`: Uninstall a package.
- `update` <package>: Update a package, if no package is specified, update all the packages.
- `list`: List all the packages you have installed.
- `sync`: Compare the packages installed on the computer with the ones stored in the file and install the missing ones.


## Example
In this example I will install `vim` (btw) and `zsh` on both of my computers
```bash
$ depot install vim
vim is now installed.

$ depot install zsh
zsh is now installed.

$ scp ~/.depot/packages user@computer:~/.depot/packages
packages file copied.

$ ssh user@computer
Welcome to computer.

$ depot sync
All packages are up to date.

$ which vim
/usr/bin/vim
```

## Supported package manager

| OS            | Manager   |
|---------------|-----------|
| Arch (btw)    | `pacman`  |
| Arch (btw)    | `yay`     |
| Debian/Ubuntu | `apt`     |
| Debian/Ubuntu | `apt-get` |
| Termux        | `pkg`     |
| Fedora        | `dnf`     |
| Alpine        | `apk`     |

The macOS package manager `brew` and Windows package managers `winget`, `choco`, and `scoop` may be supported in the future. If your package manager isn't
listed above, please [report an issue](https://github.com/Rignchen/Depot/issues/new). We will add it as soon as possible.

## License
This project is licensed under the Creative Commons NonCommercial-ShareAlike 4.0 International License - see the [LICENSE.md](LICENSE.md) file for details

