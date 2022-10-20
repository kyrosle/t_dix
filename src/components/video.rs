use dioxus::prelude::*;
use serde_json::json;
use wasm_bindgen::JsCast;
use web_sys::HtmlVideoElement;

use crate::{components::controls::Controls, VideoStream};


/// # Get from discord
///
/// ## Case code:
///
/// ```
/// #[allow(non_snake_case)]
/// fn videoNode(cx: Scope) -> Element {
///     let stream = use_ref(&cx, || None);
///     // when the element is mounted, bind the video element to the scope
///     let _ = use_future(&cx, (), |_| {
///         let stream = stream.clone();
///         async move {
///             let md = gloo_utils::window().navigator().media_devices().unwrap();
///             let mut constraints = MediaStreamConstraints::new();
///             constraints.video(&JsValue::from(true));
///             constraints.audio(&JsValue::from(true));
///             let gum = md.get_user_media_with_constraints(&constraints).unwrap();
///             let cam_video = JsFuture::from(gum).await.map(MediaStream::from);
///             if let Ok(ms) = cam_video {
///                 gloo_utils::document()
///                     .get_element_by_id("my-video")
///                     .unwrap()
///                     .dyn_into::<HtmlMediaElement>()
///                     .unwrap()
///                     .set_src_object(Some(&ms));
///                 stream.write().replace(ms);
///             }
///         }
///     });
///     cx.render(rsx! {
///         video {
///             id: "my-video",
///             autoplay: "true",
///         }
///     })
/// }
/// ```
#[allow(non_snake_case)]
pub fn Video(cx: Scope) -> Element {
    let stream = use_ref(&cx, || None);

    // let title = cx.consume_context::<String>().unwrap();
    // let title = binding.get_value();

    let _ = use_future(&cx, (), |_| {
        let mut stream = stream.clone();
        let video_constraints = json!({
            "audio": false,
            "video": {
                "facingMode": "environment",
                "width": 640,
                "height": 480,
            }
        });
        async move {
            let el = gloo_utils::document()
                .get_element_by_id("my-video")
                .unwrap()
                .dyn_into::<HtmlVideoElement>()
                .unwrap();
            let video_stream = VideoStream::new(el);
            video_stream
                .set_video_src(&video_constraints, &mut stream)
                .await;

            // let devices = Devices::load().await;
            // info!(" devices: {:#?}", devices);
        }
    });
    cx.render(rsx! {
        div {
            class: "relative",
            video {
                id: "my-video",
                class: "border border-gray-400 rounded-lg",
                autoplay: "true",
                width: "640",
                height: "480",
            }
            // p {"{title}"}
            Controls()
        }
    })
}
