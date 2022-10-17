use dioxus::prelude::UseRef;
use gloo_utils::format::JsValueSerdeExt;
use tracing::info;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{MediaStream, MediaStreamConstraints, HtmlVideoElement};
pub struct VideoStream {
    el: HtmlVideoElement,
}

impl VideoStream {
    pub fn new(el: HtmlVideoElement) -> VideoStream {
        VideoStream { el }
    }

    pub async fn set_video_src(
        &self,
        video_constraints: &serde_json::Value,
        stream: &mut UseRef<Option<MediaStream>>,
    ) {
        // get media device by js binding
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
        info!("media stream(tracing_wasm): {:?}", media_stream);
        self.el.set_src_object(Some(&media_stream));
        stream.write().replace(media_stream);
    }
}
