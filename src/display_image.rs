use crate::image_queue::ImageQueue;
use std::sync::Arc;
use wasm_bindgen::{closure::Closure, JsCast, UnwrapThrowExt};
use web_sys::{HtmlCanvasElement, ImageData, Window};
use yew::prelude::*;

fn draw_image_on_canvas(image_data: ImageData) -> String {
    let window: Window = web_sys::window().unwrap_throw();
    let canvas = window
        .document()
        .unwrap_throw()
        .create_element("canvas")
        .unwrap_throw()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap_throw();

    canvas.set_width(image_data.width());
    canvas.set_height(image_data.height());

    let ctx = canvas
        .get_context("2d")
        .unwrap_throw()
        .unwrap_throw()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap_throw();

    ctx.put_image_data(&image_data, 0.0, 0.0).unwrap_throw();

    canvas.to_data_url().unwrap_throw()
}

fn create_interval_callback(
    image_queue: Arc<ImageQueue>,
    data_url: UseStateHandle<String>,
) -> Closure<dyn FnMut()> {
    Closure::wrap(Box::new(move || {
        if let Some(image_data) = image_queue.pop() {
            let url = draw_image_on_canvas(image_data);
            data_url.set(url);
        }
    }) as Box<dyn FnMut()>)
}

#[derive(Properties, PartialEq, Clone)]
pub struct DisplayImageProps {
    pub image_queue: Arc<ImageQueue>,
}

#[function_component(DisplayImage)]
pub fn display_image(props: &DisplayImageProps) -> Html {
    let data_url = use_state(|| String::new());
    let image_queue = props.image_queue.clone();
    let data_url_cloned = data_url.clone();
    use_effect(move || {
        let window: Window = web_sys::window().unwrap_throw();
        let callback = create_interval_callback(image_queue.clone(), data_url_cloned);

        let handle = window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                10,
            )
            .unwrap_throw();

        callback.forget();

        move || {
            web_sys::window()
                .unwrap_throw()
                .clear_interval_with_handle(handle);
        }
    });

    html! {
        <img src={(*data_url).clone()} />
    }
}
