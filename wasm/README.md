# wayfolio-wasm

This directory contains the source for the WASM module of wayfolio.

## Building

```shell
~$ wasm-pack build --release --out-name wasm --target web --no-typescript --no-pack
```

See https://wasm-bindgen.github.io/wasm-pack.

This creates the following files:

```shell
~$ tree pkg
pkg
├── wasm_bg.wasm
└── wasm.js
```

These files need to be copied to the directory that contains the rest of your
generated page:

```shell
~$ cp pkg/* /path/to/the/generated/page
```

## Providing the Compendium

In order for the WASM to create cross-links to other files in your wayfolio
instance, you need to provide a compendium. The compendium contains the
available protocols, interfaces, interface members, and compositor support.

If you do not provide a compendium at build time, an empty compendium is used
and no links will be created.

To provide a compendium, write the `src/compendium_src.rs` file while building
your wayfolio site. For example:

```rust
fn write_compendium_src(compendium: &Compendium) {
    std::fs::write(
        "/path/to/wayfolio-wasm/src/compendium_src.rs",
        compendium.create_src(),
    ).unwrap();
}
```

The build steps for your site are therefore:

1. Use the wayfolio crate to generate the
   - HTML, JavaScript, and CSS files;
   - and the `compendium_src.rs` file.
2. Build the WASM module with wasm-pack.
3. Copy the files from the `pkg` directory to your page.
