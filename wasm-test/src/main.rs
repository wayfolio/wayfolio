use {
    std::slice,
    wayfolio::{
        compendium::{Compendium, Compositor, SourceInterface},
        cs::CompositorInfo,
    },
};

fn main() {
    let src = SourceInterface {
        name: "abc",
        protocol: "def",
        members: &[("x", [true, true, true])],
        compositors: &[Compositor {
            info: &CompositorInfo {
                name: "ghi",
                version: "jkl",
            },
            version: 2,
        }],
    };
    let compendium = Compendium::from_sources(slice::from_ref(&src), &[]).unwrap();
    std::fs::write(
        concat!(env!("CARGO_MANIFEST_DIR"), "/../wasm/src/compendium_src.rs"),
        compendium.create_src(),
    )
    .unwrap();
}
