use dioxus::prelude::*;

pub fn TestComponent(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);
    cx.render(rsx! {
        div { 
            "{count}"
            button {
                onclick: move |_| count += 1,
                "Add"
            }
        }
    })
}
