# Hyrax

Hyrax is a very experimental addon management tool for Godot.

# Motivation

This tool is attempting to resolve the following sore points:
- Creating templates, without inlining plugin code
- Allowing for local development of a plugin inside of a project, without stream crossing
- Bootstrapping new projects with desired plugins quickly 

##  Alternatives

- [Glam](https://github.com/henriquelalves/glam)
- [gd-plug](https://github.com/imjp94/gd-plug)
- [Godot Env](https://github.com/chickensoft-games/GodotEnv)
- [Godotons](https://github.com/Ducking-Games/godotons)

# Documentation

# Version History

# Development

## Project Structure

The project is written in rust, and is somewhat based on [Regolith](https://github.com/Bedrock-OSS/regolith)

## Running

e.g. `cargo run -q --`

e.g. `cargo run -q -- init -d "../test" --force`

