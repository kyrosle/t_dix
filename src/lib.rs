mod components;
mod devices;
mod video_stream;

pub use components::App;
use devices::Devices;
pub use video_stream::VideoStream;

#[derive(Debug, Clone)]
pub struct AppState {
    pub device_id: String,
    pub devices: Devices,
}

impl AppState {
    pub async fn new() -> Self {
        let device_id = String::from("");
        let devices = Devices::load().await;
        Self { device_id, devices }
    }
}
