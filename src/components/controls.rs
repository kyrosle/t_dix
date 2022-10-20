use dioxus::prelude::*;
use tracing::info;

use crate::{components::Context, AppState};

#[allow(non_snake_case)]
pub fn Controls(cx: Scope) -> Element {
    let state = cx.consume_context::<AppState>();
    let app_state = match state {
        None => return cx.render(rsx! { "Controls Loading"}),
        Some(state) => state,
    };
    info!("devices : {:?}", app_state);

    let force_render = cx.schedule_update_any();

    let mut title = cx.consume_context::<Context<String>>().unwrap();

    cx.render(rsx! {
        div {
            class: "absolute bottom-2  p-5 w-full",
            div {
                class: "flex justify-center",
                div {
                    class: "mb-3 xl:w-96",
                    select {
                        class: "form-select 
                        appearance-none
                        block
                        px-3
                        py-1.5
                        text-base
                        font-normal
                        text-gray-700
                        bg-white bg-clip-padding bg-no-repeat
                        border border-solid border-gray-300
                        rounded
                        transition
                        ease-in-out
                        m-0
                        focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none",
                        "aria-label": "Default select example",
                        option { value: "", "Select"}
                        option { value: "environment", "environment"}
                        option { value: "user", "User"}
                    }
                }
            }
        }
        button {
            onclick: move |_| {
                title.change_value("iiii".to_string()); 
                info!("{:?}",title.get_value());
                force_render(ScopeId(0));},
            "Change"
        }
    })
}
