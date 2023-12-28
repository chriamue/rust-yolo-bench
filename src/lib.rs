use std::sync::Arc;

use yew::prelude::*;

use crate::image_queue::ImageQueue;

pub mod bbox;
pub mod detection;
pub mod display_image;
pub mod image_queue;
pub mod video_producer;
pub mod yolo;

#[function_component]
pub fn App() -> Html {
    let video_queue = Arc::new(ImageQueue::new(3));

    html! {
        <div>
            <video_producer::VideoProducer image_queue={video_queue.clone()} />
            <display_image::DisplayImage image_queue={video_queue} />
        </div>
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub crop: Option<bool>,
}

impl Default for Config {
    fn default() -> Self {
        Config { crop: None }
    }
}
