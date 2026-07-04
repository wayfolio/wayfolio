# wayfolio

This repository contains the wayfolio crate.

Wayfolio can be used to generate HTML documentation from Wayland protocol
definitions. An instance of the generated documentation is hosted at
https://wayfolio.github.io.

## Features

- The generated documentation is fully static with no server-side component.
- References to interfaces and enums link to their definitions.
- Such references are automatically detected in descriptions.
- Globals show a compositor-support table.
- Users can paste or link a protocol file to generate docs ad-hoc.
- No JavaScript except for the ad-hoc doc generation.
- Built-in light and dark modes.

## Usage

To see how to use this crate, you can take a look at the rustdoc

```shell
~$ cargo doc --open
```

or the source code of the hosted instance:
https://github.com/wayfolio/wayfolio.github.io.

In broad strokes:

1. Create a new Rust application that depends on wayfolio:
   1. Load the protocols.
   2. Use wayfolio to write the HTML, JavaScript, and CSS files.
   3. Write the `compendium_src.rs` file used by [WASM] module.
2. Build the WASM module.
3. Copy the WASM files to your site.

Wayfolio does not care how you load the protocols. The hosted instance uses the
SQLite database from [wayland-db] but wayfolio also comes with an XML parser.

[WASM]: ./wasm/README.md
[wayland-db]: https://github.com/mahkoh/wayland-db

## License

Wayfolio is free software licensed under the GNU General Public License v3.0.
