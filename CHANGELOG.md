# Changelog

## [0.1.1](https://github.com/Rignchen/Depot/compare/v0.1.0...v0.1.1) (2024-10-12)


### Features

* add the ability to manualy specify the package manager ([9d334fb](https://github.com/Rignchen/Depot/commit/9d334fb13a53d0253e5a810e0e56f28f981a8c12))
* basic subcommand handling ([9d334fb](https://github.com/Rignchen/Depot/commit/9d334fb13a53d0253e5a810e0e56f28f981a8c12))
* change bash test script to handle command line argument ([9d334fb](https://github.com/Rignchen/Depot/commit/9d334fb13a53d0253e5a810e0e56f28f981a8c12))
* create a temporary readme ([5321dd0](https://github.com/Rignchen/Depot/commit/5321dd0f2fcdc9ea52b3bdb264bfda9bd18bc62f))
* create an enum of supported language ([7dc9d48](https://github.com/Rignchen/Depot/commit/7dc9d489ded6e4aeec1cad6b57c12bc47d917721))
* detect if the user is on linux ([7dc9d48](https://github.com/Rignchen/Depot/commit/7dc9d489ded6e4aeec1cad6b57c12bc47d917721))
* detect wich operating system the user uses ([#6](https://github.com/Rignchen/Depot/issues/6)) ([7dc9d48](https://github.com/Rignchen/Depot/commit/7dc9d489ded6e4aeec1cad6b57c12bc47d917721))
* get the default package manager from operating system ([6e712a7](https://github.com/Rignchen/Depot/commit/6e712a71d19f3b63f33c51b3dabe3731c0fc2d59))
* get the package manager ([#8](https://github.com/Rignchen/Depot/issues/8)) ([6e712a7](https://github.com/Rignchen/Depot/commit/6e712a71d19f3b63f33c51b3dabe3731c0fc2d59))
* get the package manager from all 3 possible input ([6e712a7](https://github.com/Rignchen/Depot/commit/6e712a71d19f3b63f33c51b3dabe3731c0fc2d59))
* handle command line arguments ([#7](https://github.com/Rignchen/Depot/issues/7)) ([9d334fb](https://github.com/Rignchen/Depot/commit/9d334fb13a53d0253e5a810e0e56f28f981a8c12))
* read the `/etc/os-releas` file to get the os ([7dc9d48](https://github.com/Rignchen/Depot/commit/7dc9d489ded6e4aeec1cad6b57c12bc47d917721))


### Bug Fixes

* .gitignore ([731b597](https://github.com/Rignchen/Depot/commit/731b5978e660ce6ac8d1aa1a08c9d04c8f28ff96))
* **exist:** fix bug where apt-get wouldn't be tested correctly ([#30](https://github.com/Rignchen/Depot/issues/30)) ([76534aa](https://github.com/Rignchen/Depot/commit/76534aacbe413e585f50d6f801f8363e7253e882))


### Tests

* add bash script to quickly test on all os ([7dc9d48](https://github.com/Rignchen/Depot/commit/7dc9d489ded6e4aeec1cad6b57c12bc47d917721))
* print the os name ([7dc9d48](https://github.com/Rignchen/Depot/commit/7dc9d489ded6e4aeec1cad6b57c12bc47d917721))


### Miscellaneous Chores

* **workflow:** setup action to assign reviewer ([#24](https://github.com/Rignchen/Depot/issues/24)) ([daa2dad](https://github.com/Rignchen/Depot/commit/daa2dad4ec1dea62b817d38f9f7f5127cf38a58d))


### Documentation

* add current version in readme ([795e89f](https://github.com/Rignchen/Depot/commit/795e89f2869d67848731b0ac7f071a974276cbee))
* Better README.md ([731b597](https://github.com/Rignchen/Depot/commit/731b5978e660ce6ac8d1aa1a08c9d04c8f28ff96))
* Improve README.md content ([#2](https://github.com/Rignchen/Depot/issues/2)) ([731b597](https://github.com/Rignchen/Depot/commit/731b5978e660ce6ac8d1aa1a08c9d04c8f28ff96))


### Refactors

* change `Result` type to have `DepotError` be silenced ([7dc9d48](https://github.com/Rignchen/Depot/commit/7dc9d489ded6e4aeec1cad6b57c12bc47d917721))
* **cli:** use clap's way to impose at least 1 argument when installing/removing packages ([36043d6](https://github.com/Rignchen/Depot/commit/36043d6344eda0a1778fc47e67dae8dd8ce4c472))
* move code in multiple files ([#14](https://github.com/Rignchen/Depot/issues/14)) ([de98faa](https://github.com/Rignchen/Depot/commit/de98faa2087325ad2e693f3a2bd3be74be0797f6))
* use structstruck to make the Args struct more readable ([9d334fb](https://github.com/Rignchen/Depot/commit/9d334fb13a53d0253e5a810e0e56f28f981a8c12))


### Build System

* **deps:** bump clap from 4.5.9 to 4.5.16 ([#22](https://github.com/Rignchen/Depot/issues/22)) ([62dc8b1](https://github.com/Rignchen/Depot/commit/62dc8b1d86cdeb0c8e904b712b3af0da139d49bb))


### Continuous Integration

* change token for release please ([#12](https://github.com/Rignchen/Depot/issues/12)) ([49e9df2](https://github.com/Rignchen/Depot/commit/49e9df25b258695f2303fbf462f7f7fd47690c08))
* ensure code has been formatted ([051b9eb](https://github.com/Rignchen/Depot/commit/051b9ebbf02760c6a55fe082604f50a8e0340984))
* execute code tests ([051b9eb](https://github.com/Rignchen/Depot/commit/051b9ebbf02760c6a55fe082604f50a8e0340984))
* lint code ([051b9eb](https://github.com/Rignchen/Depot/commit/051b9ebbf02760c6a55fe082604f50a8e0340984))
* setup dependabot ([#5](https://github.com/Rignchen/Depot/issues/5)) ([5e43d0d](https://github.com/Rignchen/Depot/commit/5e43d0dde6224f2cd2ad9146fd7f73fffdcfb89c))
* setup release please ([#4](https://github.com/Rignchen/Depot/issues/4)) ([795e89f](https://github.com/Rignchen/Depot/commit/795e89f2869d67848731b0ac7f071a974276cbee))
* test code before pushing on main ([#3](https://github.com/Rignchen/Depot/issues/3)) ([051b9eb](https://github.com/Rignchen/Depot/commit/051b9ebbf02760c6a55fe082604f50a8e0340984))
* test if code compiles ([051b9eb](https://github.com/Rignchen/Depot/commit/051b9ebbf02760c6a55fe082604f50a8e0340984))
