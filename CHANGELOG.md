# Changelog

## [0.0.2](https://github.com/Rignchen/Depot/compare/v0.0.1...v0.0.2) (2024-07-12)


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


### Tests

* add bash script to quickly test on all os ([7dc9d48](https://github.com/Rignchen/Depot/commit/7dc9d489ded6e4aeec1cad6b57c12bc47d917721))
* print the os name ([7dc9d48](https://github.com/Rignchen/Depot/commit/7dc9d489ded6e4aeec1cad6b57c12bc47d917721))


### Documentation

* add current version in readme ([795e89f](https://github.com/Rignchen/Depot/commit/795e89f2869d67848731b0ac7f071a974276cbee))
* Better README.md ([731b597](https://github.com/Rignchen/Depot/commit/731b5978e660ce6ac8d1aa1a08c9d04c8f28ff96))
* Improve README.md content ([#2](https://github.com/Rignchen/Depot/issues/2)) ([731b597](https://github.com/Rignchen/Depot/commit/731b5978e660ce6ac8d1aa1a08c9d04c8f28ff96))


### Refactors

* change `Result` type to have `DepotError` be silenced ([7dc9d48](https://github.com/Rignchen/Depot/commit/7dc9d489ded6e4aeec1cad6b57c12bc47d917721))
* use structstruck to make the Args struct more readable ([9d334fb](https://github.com/Rignchen/Depot/commit/9d334fb13a53d0253e5a810e0e56f28f981a8c12))


### Continuous Integration

* ensure code has been formatted ([051b9eb](https://github.com/Rignchen/Depot/commit/051b9ebbf02760c6a55fe082604f50a8e0340984))
* execute code tests ([051b9eb](https://github.com/Rignchen/Depot/commit/051b9ebbf02760c6a55fe082604f50a8e0340984))
* lint code ([051b9eb](https://github.com/Rignchen/Depot/commit/051b9ebbf02760c6a55fe082604f50a8e0340984))
* setup dependabot ([#5](https://github.com/Rignchen/Depot/issues/5)) ([5e43d0d](https://github.com/Rignchen/Depot/commit/5e43d0dde6224f2cd2ad9146fd7f73fffdcfb89c))
* setup release please ([#4](https://github.com/Rignchen/Depot/issues/4)) ([795e89f](https://github.com/Rignchen/Depot/commit/795e89f2869d67848731b0ac7f071a974276cbee))
* test code before pushing on main ([#3](https://github.com/Rignchen/Depot/issues/3)) ([051b9eb](https://github.com/Rignchen/Depot/commit/051b9ebbf02760c6a55fe082604f50a8e0340984))
* test if code compiles ([051b9eb](https://github.com/Rignchen/Depot/commit/051b9ebbf02760c6a55fe082604f50a8e0340984))
