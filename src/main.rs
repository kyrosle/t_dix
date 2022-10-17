use dioxus::prelude::*;
use t_dix::Video;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            div{
                class: "container p-2",
                Video {}
            }
        }
    })
}
