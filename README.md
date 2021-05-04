# pipboy
A mod manager for Fallout: New Vegas

## Table of Contents

1. [Configuration](#configuration)
   1. [Custom Directory](#custom-directory)
   2. [Configuration File Values](#configuration-file-values)
2. [Profiles](#profiles)
3. [Caches](#caches)
4. [Installing Mods](#installing-mods)
   1. [Force installation](#force-installation)
   2. [Mod file ownership](#mod-file-ownership)
5. [Uninstalling Mods](#uninstalling-mods)

## Configuration

### Custom Directory

The default configuration file for pipboy is stored at `~/.pipboy/config`, but the `~/.pipboy` directory can be replaced with a custom path using the `-c` argument.
```
pipboy 0.1.0
Aayla Semyonova <aayla@aayla.dev>
A Mod Manager for Fallout New Vegas

USAGE:
    pipboy [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Increase verbosity of output

OPTIONS:
    -c <config>        Path to config directory [default: ~/.pipboy]

SUBCOMMANDS:
    cache        
    help         Prints this message or the help of the given subcommand(s)
    install      Install a mod
    profile      
    uninstall    Uninstall a mod
```
### Configuration File Values

The default configuration file for pipboy is as follows:
```conf
current_profile = 'Fallout New Vegas'
repository_list = 'pipboy.aayla.dev'
```
The meaning of each value is:
1. current_profile: The current profile in use by pipboy. This is primarly used to maintain state between command executions and probably shouldn't be edited manually.
2. repository_list: The list of remote repositories to search for packages in.

## Profiles
```
pipboy-profile 

USAGE:
pipboy profile [SUBCOMMAND]

FLAGS:
-h, --help       Prints help information
-V, --version    Prints version information

SUBCOMMANDS:
create    Create a new profile
help      Prints this message or the help of the given subcommand(s)
list      List all profiles
ls        Alias of list
remove    Remove a profile
rm        Alias of remove
select    Select a profile
```