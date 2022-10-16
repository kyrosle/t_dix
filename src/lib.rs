use dioxus::prelude::{Component, Element, UseRef, VNode};
use gloo_utils::format::JsValueSerdeExt;
use tracing::info;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{MediaStream, MediaStreamConstraints};
pub struct VideoStream {
    el: Component,
}

impl VideoStream {
    pub fn new(el: Component) -> VideoStream {
        VideoStream { el }
    }

    pub async fn set_video_src(&self, video_constraints: &serde_json::Value) {
        let window = web_sys::window().expect("no global `window` exists");
        let navigator = window.navigator();
        let devices = navigator
            .media_devices()
            .expect("no `navigator.media_devices` exists");
        info!("devices (tracing_wasm) : {:?}", devices);
        web_sys::console::log_1(&devices);

        let mut constraints = MediaStreamConstraints::new();
        constraints.video(&JsValue::from_serde(video_constraints).unwrap());
        constraints.audio(&false.into());
        let media = JsFuture::from(
            devices
                .get_user_media_with_constraints(&constraints)
                .unwrap(),
        )
        .await
        .unwrap();
        let media_stream = media.unchecked_into::<MediaStream>();
        // let media_stream = media.dyn_into();
        info!("media stream(tracing_wasm): {:?}", media_stream);
    }
}
