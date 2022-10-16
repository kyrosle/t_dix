use dioxus::prelude::*;
use serde_json::json;
use t_dix::VideoStream;
use wasm_bindgen::JsValue;
use web_sys::MediaStreamConstraints;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus::web::launch(videoNode);
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
fn videoNode(cx: Scope) -> Element {
    let stream = use_ref(&cx, || None);

    // when the element is mounted, bind the video element to the scope
    use_future(&cx, move || {
        let stream = stream.clone();
        async move {
            let md = gloo_utils::window().navigator().media_devices().unwrap();
            let mut constraints = MediaStreamConstraints::new();
            constraints.video(&JsValue::from(true));
            constraints.audio(&JsValue::from(true));
            let gum = md.get_user_media_with_constraints(&constraints).unwrap();
            let cam_video = JsFuture::from(gum).await.map(MediaStream::from);

            if let Ok(ms) = cam_video {
                gloo_utils::document()
                    .get_element_by_id("my-video")
                    .unwrap()
                    .dyn_into::<HtmlMediaElement>()
                    .unwrap()
                    .set_src_object(Some(&ms));
                stream.write().replace(ms);
            }
        }
    });

    cx.render(rsx! {
        video { id: "my-video", auto_play: "true" /* autoplay is very important */ }
    })
}

#[allow(non_snake_case)]
fn Video(cx: Scope) -> Element {
    let video_node: Component = |cx| {
        cx.render(rsx! {
            video {
            id: "my-video",
            class: "border border-gray-400 rounded-lg",
            autoplay: "true",
            width: "1280",
            height: "720",
        }})
    };
    let video_ref = use_ref(&cx, || video_node);
    let _ = use_future(&cx, (), |_| async move {
        let video = VideoStream::new(video_node);
        video
            .set_video_src(&json!({
                "audio": false,
                "video": {
                    "facingMode": "environment",
                    "width": 640,
                    "height": 480,
                }
            }))
            .await;
    });
    cx.render(rsx! {
        div {
        }
    })
}
