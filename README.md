# Spigotfly CLI

[![Crates.io](https://img.shields.io/crates/v/spigotfly)](https://crates.io/crates/spigotfly)

The Spigotfly command line interface (CLI) permits to search and download spigot resources.
The Spigotfly CLI allows you to

- Search a resource by its name or tags with some optional parameters
- Download a resource

## Installation
Spigotfly builds can be downloaded in the GitHub release page.
Alternatively, you could install it using the Cargo package manager.

`spigotfly` is written in [Rust](https://www.rust-lang.org/), and thus requires the Rust toolchain to compile.
Follow the instructions at <https://www.rust-lang.org/tools/install> to install the toolchain.
Once that is complete, execute the following command:
```bash
cargo install spigotfly
```

## Usage
```bash
C:\Users\User\Desktop>spigotfly
Command line tool for downloading spigot plugins

Usage: spigotfly <COMMAND>

Commands:
  search    Search a resource
  download  Download a resource by id
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

C:\Users\User\Desktop>spigotfly search -h
Search a resource

Usage: spigotfly search [OPTIONS] [QUERY]...

Arguments:
  [QUERY]...

Options:
  -f, --field <FIELD>  Field to search in [default: name] [possible values: name, tag]
  -s, --size <SIZE>    Size of the returned resources [default: 10]
  -p, --page <PAGE>    Page index [default: 0]
  -h, --help           Print help (see more with '--help')

C:\Users\User\Desktop>spigotfly download -h
Download a resource by id

Usage: spigotfly download [OPTIONS] <ID>

Arguments:
  <ID>  Resource id to download

Options:
  -o, --output <OUTPUT>  Plugin jar file name. The id is the default
  -h, --help             Print help

C:\Users\User\Desktop>spigotfly search pvp
.-----------------------------------------------------------.
| Id   | Name                          | Downloads | Rating |
| 845  | PvPManager Lite               | 115520    | OOOOO  |
| 79   | LibsHungergames - MCPVP style | 45863     | OOOOO  |
| 1370 | Soups for PvP                 | 1988      | OOO    |
| 1424 | esKitPvP                      | 777       | OOO    |
| 43   | PVPUtils                      | 630       | OOOO   |
| 853  | Kangaroo - McPvP              | 610       | OOOO   |
| 997  | LibsHungergames - McpvpCake   | 261       |        |
| 1425 | PvPKits                       | 258       |        |
| 1387 | kUnbreakingPvPItems           | 230       | OOOOO  |
| 1018 | KingPvp                       | 215       |        |
'-----------------------------------------------------------'

C:\Users\User\Desktop>spigotfly download 845 -o PvpPlugin

C:\Users\User\Desktop>
```
