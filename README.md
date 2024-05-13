# Spigotfly CLI

[![Crates.io](https://img.shields.io/crates/v/spigotfly)](https://crates.io/crates/spigotfly)

The Spigotfly command line interface (CLI) permits to search and download spigot resources.
The Spigotfly CLI allows you to

- Search a resource by its name or tags with some optional parameters
- Download a resource

## Installation
`spigotfly` is written in [Rust](https://www.rust-lang.org/), and thus requires the Rust toolchain to compile.
Follow the instructions at <https://www.rust-lang.org/tools/install> to install the toolchain.
Once that is complete, execute the following command:
### Cargo
```bash
cargo install spigotfly
```

## Usage
```bash
C:\Users\User>spigotfly
Command line tool for downloading spigot plugins

Usage: spigotfly <COMMAND>

Commands:
  search    Search a resource
  download  Download a resource by id
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

C:\Users\User>
```
