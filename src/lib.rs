pub mod app;
pub mod bbox;
pub mod detection;
pub mod display_image;
pub mod image_queue;
pub mod model;
pub mod model_selection;
pub mod pipeline;
pub mod video_producer;
pub mod yolo;

pub use app::App;
pub use model::Model;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub crop: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        Config { crop: None }
    }
}
