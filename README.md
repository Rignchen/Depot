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
package list in the same place as your dotfiles.

## Installation

### Pre-built binaries
You can download the latest release from the [releases page](https://github.com/Rignchen/Depot/releases/latest).\
After downloading the binary, you can move it to a directory in your PATH.

### Package manager
Package manager support is coming soon.\
Consider building from source or using the pre-built binaries for now.

### Building from source

```bash
git clone git@github.com:Rignchen/Depot.git
cd Depot
cargo build --release
mv target/release/depot /usr/local/bin
```
Please note that you need to have [Rust](https://www.rust-lang.org/tools/install) installed to build the project.

## Configuration
Depot will try to guess your package manager based on your operating system. However, sometimes you may want to specify it yourself.
For example, as I use Arch (btw), I may want to use `yay` instead of `pacman`.\
To do so you have 2 options:
1. Set the environment variable `DEPOT_PM` to the name of your package manager
2. Add the flag `--pm <package_manager>` when running a command

## Usage
Depot has 5 commands:
- `install <package>`: Install a package.
- `uninstall <package>`: Uninstall a package.
- `update [package]`: Update a package, if no package is specified, update all the packages.
- `list`: List all the packages you have installed.
- `sync`: Compare the packages installed on the computer with the ones stored in the file and install the missing ones.


## Example
In this example I will install `vim` and `zsh` on both of my computers.\
I use `scp` to copy the package list file from one computer to the other.\
Then I use `ssh` to connect to the other computer and run `depot sync` to install the missing packages on the other computer.
```bash
$ depot install vim
vim is now installed.

$ depot install zsh
zsh is now installed.

$ scp ~/.depot/packages user@computer:~/.depot/packages
README.md                                     100% 2287     2.2MB/s   00:00    

$ ssh user@computer 
$ whoami
user

$ depot sync
Packages synced with config.

$ which vim
/usr/bin/vim
```

## Supported package manager

Package managers listed below are supported by Depot:

| OS/Linux Distribution | `pacman` | `yay` | `apt` | `apt-get` | `pkg` | `dnf` | `apk` |
|-----------------------|----------|-------|-------|-----------|-------|-------|-------|
| Arch (btw)            | ✔        | ✔     | X     | X         | X     | X     | X     |
| Debian/Ubuntu         | X        | X     | ✔     | ✔         | X     | X     | X     |
| Android (Termux)      | X        | X     | X     | X         | ✔     | X     | X     |
| Fedora                | X        | X     | X     | X         | X     | ✔     | X     |
| Alpine                | X        | X     | X     | X         | X     | X     | ✔     |

The macOS package manager `brew` and Windows package managers `winget`, `choco`, and `scoop` may be supported in the future. If your package manager isn't
listed above, please [report an issue](https://github.com/Rignchen/Depot/issues/new). We will add it as soon as possible.

## License
This project is licensed under the [Creative Commons NonCommercial-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-nc-sa/4.0/) - see the [LICENSE.md](LICENSE.md) file for details.

