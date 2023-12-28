use std::sync::Arc;

use wasm_bindgen::{closure::Closure, JsCast, UnwrapThrowExt};
use web_sys::Window;
use yew::prelude::*;

use crate::{image_queue::ImageQueue, pipeline::Pipeline};

pub mod bbox;
pub mod detection;
pub mod display_image;
pub mod image_queue;
pub mod model_selection;
pub mod pipeline;
pub mod video_producer;
pub mod yolo;

#[function_component]
pub fn App() -> Html {
    let video_queue = use_state(|| Arc::new(ImageQueue::new_with_id(1, 3)));
    let processed_queue = use_state(|| Arc::new(ImageQueue::new(3)));
    let selected_model = use_state(|| Model::None);
    let pipeline = use_state(|| {
        Arc::new(Pipeline::new(
            Model::None,
            (*video_queue).clone(),
            (*processed_queue).clone(),
        ))
    });

    let on_model_selected = {
        let selected_model = selected_model.clone();
        let pipeline = pipeline.clone();
        Callback::from(move |model: Model| {
            selected_model.set(model);
            pipeline.set_model(model);
        })
    };

    use_effect(move || {
        let pipeline_clone = pipeline.clone();
        let window: Window = web_sys::window().unwrap_throw();
        let closure = Closure::wrap(Box::new(move || {
            if let Err(e) = pipeline_clone.process() {
                log::error!("Error processing pipeline: {:?}", e);
            }
        }) as Box<dyn FnMut()>);

        let interval_id = window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                10,
            )
            .unwrap_throw();

        closure.forget();

        move || {
            window.clear_interval_with_handle(interval_id);
        }
    });

    html! {
        <div>
            <model_selection::ModelSelection on_select={on_model_selected} />
            <video_producer::VideoProducer image_queue={(*video_queue).clone()} />
            <display_image::DisplayImage image_queue={(*processed_queue).clone()} />
        </div>
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Model {
    None,
    #[cfg(feature = "tract")]
    Tract,
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
