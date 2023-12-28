use crate::image_queue::ImageQueue;
use std::sync::Arc;
use wasm_bindgen::{closure::Closure, JsCast, JsValue, UnwrapThrowExt};
use web_sys::{HtmlCanvasElement, ImageData, Window};
use yew::prelude::*;

fn draw_image_on_canvas(image_data: ImageData, last_update: f64) -> (String, f64) {
    let window: Window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap_throw();
    let canvas = document
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
    let now = window.performance().unwrap_throw().now();
    let new_last_update = if last_update > 0.0 {
        let elapsed = now - last_update;
        let fps = 1000.0 / elapsed;
        ctx.set_font("20px Arial");
        ctx.set_fill_style(&JsValue::from_str("red"));
        ctx.fill_text(&format!("FPS: {:.2}", fps), 10.0, 30.0)
            .unwrap_throw();
        now
    } else {
        now
    };

    (canvas.to_data_url().unwrap_throw(), new_last_update)
}

fn create_interval_callback(
    image_queue: Arc<ImageQueue>,
    data_url: UseStateHandle<String>,
    set_last_update: UseStateHandle<f64>,
) -> Closure<dyn FnMut()> {
    Closure::wrap(Box::new(move || {
        if let Some(image_data) = image_queue.pop() {
            let (url, new_last_update) = draw_image_on_canvas(image_data, *set_last_update);
            data_url.set(url);
            set_last_update.set(new_last_update);
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
    let last_update = use_state(|| 0.0);
    let image_queue = props.image_queue.clone();
    let data_url_cloned = data_url.clone();
    let last_update_cloned = last_update.clone();

    use_effect(move || {
        let window: Window = web_sys::window().unwrap_throw();
        let callback =
            create_interval_callback(image_queue.clone(), data_url_cloned, last_update_cloned);

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
