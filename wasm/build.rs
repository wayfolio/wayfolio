use std::fs;

const COMPENDIUM_SRC: &str = "src/compendium_src.rs";

fn main() {
    println!("cargo:rerun-if-changed={}", COMPENDIUM_SRC);
    if !fs::exists(COMPENDIUM_SRC).unwrap() {
        fs::copy("src/compendium_src_def.rs", COMPENDIUM_SRC).unwrap();
    }
}
