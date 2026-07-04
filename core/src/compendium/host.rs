use {
    crate::compendium::Compendium,
    std::{collections::HashMap, fmt::Write},
};

impl<'a> Compendium<'a> {
    /// Serializes the compendium into Rust source code.
    ///
    /// See the README.md of the WASM module for when and how to use this.
    pub fn create_src(&self) -> String {
        let mut compositors = HashMap::new();
        for interface in self.interfaces.values() {
            for c in &interface.compositors {
                compositors.insert(c.info.name, c.info.version);
            }
        }
        let mut compositors: Vec<_> = compositors.into_iter().collect();
        compositors.sort_by_key(|c| c.0);

        let mut res = String::new();
        macro_rules! wl {
            ($($tt:tt)*) => {
                writeln!(res, $($tt)*).unwrap()
            };
        }

        wl!(r#"use wayfolio::compendium::*;"#);
        wl!();
        wl!(r#"#[allow(non_upper_case_globals)]"#);
        wl!(r#"mod compositors {{"#);
        wl!(r#"    use wayfolio::cs::*;"#);
        wl!();
        for (name, version) in compositors {
            wl!(r#"    pub(crate) static {name}: CompositorInfo<'_> = CompositorInfo {{"#);
            wl!(r#"        name: "{name}","#);
            wl!(r#"        version: "{version}","#);
            wl!(r#"    }};"#);
        }
        wl!(r#"}}"#);
        wl!();
        wl!(r#"pub(crate) static SOURCES: &[SourceInterface<'_>] = &["#);
        let mut interfaces: Vec<_> = self.interfaces.iter().collect();
        interfaces.sort_by_key(|m| m.0);
        for (name, def) in interfaces {
            wl!(r#"    SourceInterface {{"#);
            wl!(r#"        name: "{name}","#);
            wl!(r#"        protocol: "{}","#, def.protocol);
            wl!(r#"        members: &["#);
            let mut members: Vec<_> = def.members.iter().collect();
            members.sort_by_key(|m| m.0);
            for (name, def) in members {
                wl!(r#"            ("{name}", {:?}),"#, def.0);
            }
            wl!(r#"        ],"#);
            wl!(r#"        compositors: &["#);
            for c in &def.compositors {
                wl!(r#"            Compositor {{"#);
                wl!(r#"                info: &compositors::{},"#, c.info.name);
                wl!(r#"                version: {},"#, c.version);
                wl!(r#"            }},"#);
            }
            wl!(r#"        ],"#);
            wl!(r#"    }},"#);
        }
        wl!(r#"];"#);

        res
    }
}
