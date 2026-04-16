# pokeget-rs (with Mega Pokemon support)
A fork of [talwat/pokeget-rs](https://github.com/talwat/pokeget-rs) with added support for random mega pokemon.

## New Features
- `pokeget random-mega` — displays a random mega pokemon
- All 46 mega-capable pokemon are supported, including both mega-x and mega-y forms for Charizard and Mewtwo

### Examples
#### Random mega pokemon
`pokeget random-mega`

#### Specific mega forms
`pokeget charizard --mega-x`
`pokeget charizard --mega-y`
`pokeget mewtwo --mega-x`
`pokeget mewtwo --mega-y`
`pokeget venusaur --mega`

---

## Usage
`pokeget <pokemon>`

For more info, run `pokeget --help`.

## Project status
This is a fork of pokeget-rs. The upstream project has decided to stop adding or modifying sprites.
Bug fixes will still be applied. This fork will be kept up to date with upstream where possible.

### .bashrc
If you're using pokeget on shell startup, such as in `.bashrc`,
then instead of running `pokeget <pokemon>`, you can write the output
to a file by doing: `pokeget <pokemon> > file.txt`
and then have something like `cat file.txt` in your bashrc.
This makes your shell initialization practically instant, but obviously
won't work with random pokemon. pokeget is already fairly fast,
so using it on shell initialization is also not a very large bottleneck.

### Examples
#### Using multiple pokemon
`pokeget bulbasaur pikachu random`

#### Using pokedex ID's
`pokeget 1 2 3`

#### Using alternative forms
`pokeget raichu sandslash meowth --alolan`

#### Using regions
`pokeget kanto`

## Installation

### Git *(recommended for this fork)*
```sh
git clone --recurse-submodules https://github.com/MinecraftNerd123/pokeget-rs.git
cd pokeget-rs
cargo build --release
mv target/release/pokeget ~/.cargo/bin/pokeget
```

### Cargo (upstream version, does not include mega support)
```sh
cargo install pokeget
```
and making sure `$HOME/.cargo/bin` is added to `$PATH`.

### Adding a directory to $PATH
#### Bash & Zsh
Append this to your `.bashrc` or `.zshrc`:
```sh
export PATH="<path>:$PATH"
```

#### Fish
Run this in your CLI:
```sh
fish_add_path <path>
```

## Updating
Run `git pull` on the repository and then recompile:
```sh
git pull
cargo build --release
mv target/release/pokeget ~/.cargo/bin/pokeget
```

## Why this fork?
The original pokeget-rs is great but doesn't support random mega pokemon. This fork adds a curated list of all 46 mega-capable pokemon and lets you get a random one with a single command.

## Credits
- Original project by [talwat](https://github.com/talwat/pokeget-rs)
- Sprites from [pokesprite](https://github.com/msikma/pokesprite)
- Sprites are embedded into the binary so pokeget won't download them at runtime

## License
MIT — same as the original project.
