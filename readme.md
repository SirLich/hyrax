# Regolite

Regolite is an implementation of [Regolith](https://github.com/Bedrock-OSS/regolith) written in rust.

## Design Goals

Regolite is predominantly an educational project, to teach myself rust. Beyond this primary goal, I would like for Regolite to be
less MC focused than Regolith. So while it will have 1:1 interop  with Regolith (at least for supported features), I will additionally be supporting
extensions than make it more suitable for non-MC projects. For example the `project` folder will be used as the source of temp, rather than `packs/RP` and `packs/BP`. 

# Running

e.g. `cargo run -q -- init -d "../test" --force`