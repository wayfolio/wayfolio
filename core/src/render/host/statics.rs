pub(crate) struct Static {
    pub(crate) name: &'static str,
    pub(crate) body: &'static str,
}

macro_rules! statics {
    ($($ident:ident, $name:expr;)*) => {
        $(
            pub(crate) const $ident: Static = Static {
                name: $name,
                body: include_str!(concat!("statics/", $name)),
            };
        )*
    };
}

statics! {
    COMMON_CSS, "common.css";
    FAVICON_SVG, "favicon.svg";
    INDEX_CSS, "index.css";
    PROTOCOL_CSS, "protocol.css";
    RENDER_CSS, "render.css";
    RENDER_JS, "render.js";
    RENDER_COMMON_JS, "render_common.js";
    RENDER_URL_JS, "render_url.js";
}
