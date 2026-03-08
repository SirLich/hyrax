# Hyrax

Hyrax is a technology agnostic git-based dependency manager, written in Rust. You can use it to pull repositories or repository subfolders into your project, based on a .toml file.

## Motivation

I intend to use this tool for managing Godot addons, though it can be used for other purposes. In Godot, it's quite common for addons to be published to git with this form:

```
README.md
random_stuff/...
addons/my_addon_name/...
```

Of this repo, only the contents of `addons/my_addon_name/` is desired. Hyrax allows you to easily pull in just this folder, and inline it into the repository.

## Usage and Examples

Download Hyrax from the [https://github.com/SirLich/hyrax/releases]. Open a terminal in the root of your project, and run `hyrax` to confirm that it's installed.

Run `hyrax add <url> <destination>` to add a dependency to your project. This will create or edit a `hyrax.toml` file. You can optionally use `-s` to set source, and `-v` to set version. You can open this file and adjust the package settings manually if needed.

Here is an example, showing how you can import the Godot SVG icons into your project:

```toml
[[dependencies]]
name = "Godot Icons"                          # Friendly name for the dependency
url = "https://github.com/godotengine/godot"  # The git repo you wish to install from
source = "editor/icons"                       # The folder or file you wish use
destination = "test/icons"                    # The path in your project where the dependency will be installed. Normally an empty directory.
version = "master"                            # The branch, tag, or commit SHA you want to reference
version_lock = "..."                          # Set automatically by Hyrax: Represents the actually installed version.
```

Run `hyrax sync` to install the dependencies. Dependencies without a `source` will install the entire repository, otherwise it will install just the requested folder or file. After installing, a `version_lock` field will be added to each dependency. This field tracks what commit was *actually* installed. Future syncs will use the locked version by default.

## Updating Dependencies

You can always manually update dependencies by editing the `hyrax.toml` file, and then re-running `hyrax sync`. You can also run `hyrax check` to see if any updates are available, or `hyrax update` to bump dependencies automatically.

##  Alternatives

Here are some alternatives that I considered, and rejected:

- [Glam](https://github.com/henriquelalves/glam)
- [gd-plug](https://github.com/imjp94/gd-plug)
- [Godot Env](https://github.com/chickensoft-games/GodotEnv)
- [Godotons](https://github.com/Ducking-Games/godotons)
- [gplug](https://github.com/Ducking-Games/gplug)
- [GPM](https://github.com/aerys/gpm)
- [Gitpkg](https://github.com/ramasilveyra/gitpkg)

# Version History


# Development

## Project Structure

The project is written in rust, and is somewhat based on [Regolith](https://github.com/Bedrock-OSS/regolith), or indeed my partial rust reimplementation [Regolite](https://github.com/SirLich/regolite).

## Running

e.g. `cargo run -q --`

e.g. `cargo run -q -- sync`

