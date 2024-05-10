# steam-playtime-converter

Converts playtime from hours to other units.

## Installation
**Note:** You need to have [Rust](https://www.rust-lang.org/tools/install) installed. You will also need to move the binary to a directory in your PATH, which `cargo install` attempts to do automatically, but may fail in doing.

```bash
cargo install --git https://github.com/uptudev/steam-playtime-converter.git
```

## Usage
```bash
steam_playtime_converter -H 12345
```
omitting arguments prompts for input.
```bash
steam_playtime_converter 
```

![Example output](https://i.imgur.com/TtX13Du.png)
