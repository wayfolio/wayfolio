use hypertext::prelude::*;

pub(crate) fn wrapper(
    nested: bool,
    stylesheets: &[&str],
    head: impl Renderable,
    body: impl Renderable,
) -> impl Renderable {
    let prefix = match nested {
        true => "../",
        false => "",
    };
    maud! {
        !DOCTYPE
        html lang="en" {
            head {
                meta http-equiv="content-type" content="text/html; charset=UTF-8";
                meta name="viewport" content="width=device-width,initial-scale=1.0";
                link rel="icon" type="image/svg+xml" href={ (prefix) "assets/favicon.svg" };
                @for v in stylesheets {
                    link rel="stylesheet" href={ (prefix) "assets/" (v) };
                }
                (head)
            }
            body #top {
                main {
                    (body)
                }
            }
        }
    }
}
