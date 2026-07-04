use hypertext::prelude::*;

pub(crate) mod protocol_toc;
pub(crate) mod protocols;
pub(crate) mod wrapper;

pub(crate) fn index_link(nested: bool) -> impl Renderable {
    let url = match nested {
        true => "../index.html",
        false => "index.html",
    };
    maud! {
        a .generic-link href={url} { "Go to index" }
    }
}
