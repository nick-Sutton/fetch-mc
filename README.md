# Fetch-MC
### About:
Fetch-MC is a terminal-based Minecraft Mod management utility for updating mods, resource packs, and shaders from Modrinth. 
Fetch-MC downloads the latest compatible version of mods based on a specified Minecraft version.

### Download
Fetch-MC can be installed using the [Rust](https://rustup.rs/) toolchain from [crates.io](https://crates.io/crates/fetch-mc) or by installing the binaries.
* **Rust Installation**
  - Install the [Rust](https://rustup.rs/) toolchain
  -  Run the following cargo command: `cargo install fetch-mc`
* **Binary Installation**
  - TODO

### Usage:
* Fetch mods for the latest Minecraft version:        `$ fetch-mc`
* Fetch mods for a specific Minecraft version:        `$ fetch-mc -v 1.21.1`
<!--* Specify output directory:                           `$ fetch-mc -d ./example/directory/`-->
* Specify mod list directory:                         `$ fetch-mc -l ./modlist/mods.toml`
  
### [Mod List Example:](example/mods.toml)
```toml
# ./mods.toml

# Mods
mods = [
    "sodium",
    "iris",
    "lithium",
    "indium",
    "fabric-api",
]

# Resource packs
resourcepacks = [
    "faithful-32x",
    "fresh-animations",
]

# Shaders
shaders = [
    "complementary-reimagined",
    "bsl-shaders",
]
```

