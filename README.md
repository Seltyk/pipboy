# pipboy
A mod manager for Fallout: New Vegas

## Table of Contents

1. [Configuration](#configuration)
   1. [Custom Directory](#custom-directory)
   2. [Configuration File Values](#configuration-file-values)
2. [Profiles](#profiles)
    1. [Profile Commands](#profile-commands)
    2. [Profile File Values](#profile-file-values)
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

### Profile Commands
Several profile management commands are available by passing the `profile` subcommand.

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

#### Create

Use this command to create a new profile
```
pipboy-profile-create 
Create a new profile

USAGE:
    pipboy profile create <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <name>    Name of the new profile
```

#### List

Passing the command `pipboy profile ls` will list all profiles and mark the one currently in use. This command does not take any extra arguments.
```
Available profiles:
Profile: "Fallout New Vegas" [*]
Profile: "profile2"
```

#### Remove

Use this command to remove a profile. The profile to remove is a positional argument.
```
pipboy-profile-remove 
Remove a profile

USAGE:
    pipboy profile remove <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <name>    The profile to remove
```

#### Select

Use this command to switch between profiles. The profile to switch to is a positional argument.
```
pipboy-profile-select 
Select a profile

USAGE:
    pipboy profile select <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <name>    Profile to switch to
```

### Profile File Values

The default values of a profile are as follows
```
install_path = 'path/to/fallout/install/'
enabled_mods = ''
game = 'Fallout: New Vegas'
```
The meaning of each value is:
1. install_path: The path to the installation of the modded game. From my configuration as an example, `/mnt/Games/SteamLibrary/steamapps/common/Fallout New Vegas`
2. enabled_mods: Currently unimplemented
3. game: The game associated with this profile. By default, Fallout: New Vegas as that is the game pipboy was designed with in mind.