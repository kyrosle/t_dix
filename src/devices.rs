use std::ops::Deref;

use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{MediaDeviceInfo, MediaDeviceKind, MediaDevices};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Devices(Vec<Device>);

impl Deref for Devices {
    type Target = Vec<Device>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Iterator for Devices {
    type Item = Device;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl Devices {
    pub async fn load() -> Self {
        let devices = Self::get_media_devices();
        let all_devices = JsFuture::from(devices.enumerate_devices().unwrap())
            .await
            .unwrap();
        Self::from(&all_devices)
    }
    pub fn video_devices(&self) -> impl Iterator<Item = &Device> {
        self.iter_by_kind(MediaDeviceKind::Videoinput)
    }
    pub fn audio_devices(&self) -> impl Iterator<Item = &Device> {
        self.iter_by_kind(MediaDeviceKind::Audioinput)
    }
    pub fn get_media_devices() -> MediaDevices {
        let window = web_sys::window().expect("no global `window` exists");
        let navigator = window.navigator();

        // return devices
        navigator
            .media_devices()
            .expect("no `navigator.media_devices` exists")
    }
    fn iter_by_kind(&self, kind: MediaDeviceKind) -> impl Iterator<Item = &Device> {
        self .iter()
            .filter(move |d| d.kind == kind)
    }
}

impl From<&JsValue> for Devices {
    fn from(v: &JsValue) -> Self {
        Self(match js_sys::try_iter(v) {
            Ok(Some(v)) => v
                .into_iter()
                .filter(|item| item.is_ok())
                .map(|v| Device::from(v.unwrap()))
                .collect::<Vec<Device>>(),
            _ => Default::default(),
        })
    }
}
impl From<JsValue> for Device {
    fn from(v: JsValue) -> Self {
        let device = v.unchecked_into::<MediaDeviceInfo>();
        Device {
            kind: device.kind(),
            label: device.label(),
            id: device.device_id(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Device {
    pub kind: MediaDeviceKind,
    pub label: String,
    pub id: String,
}
