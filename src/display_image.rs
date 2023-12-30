use std::{cell::RefCell, sync::Arc};

use super::image_queue::ImageQueue;
use wasm_bindgen::{closure::Closure, JsCast, JsValue, UnwrapThrowExt};
use web_sys::{HtmlCanvasElement, ImageData, Window};
use yew::prelude::*;

fn draw_image_on_canvas(
    canvas: &HtmlCanvasElement,
    image_data: ImageData,
    last_update: f64,
) -> f64 {
    canvas.set_width(image_data.width());
    canvas.set_height(image_data.height());

    let ctx = canvas
        .get_context("2d")
        .unwrap_throw()
        .unwrap_throw()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap_throw();

    ctx.put_image_data(&image_data, 0.0, 0.0).unwrap_throw();
    let window: Window = web_sys::window().unwrap_throw();
    let now = window.performance().unwrap_throw().now();
    if last_update > 0.0 {
        let elapsed = now - last_update;
        let fps = 1000.0 / elapsed;
        ctx.set_font("20px Arial");
        ctx.set_fill_style(&JsValue::from_str("red"));
        ctx.fill_text(&format!("FPS: {:.2}", fps), 10.0, 30.0)
            .unwrap_throw();
    }

    now
}

fn create_interval_callback(
    image_queue: Arc<ImageQueue>,
    canvas_ref: NodeRef,
    last_update: Arc<RefCell<f64>>,
) -> Closure<dyn FnMut()> {
    Closure::wrap(Box::new(move || {
        if let Some(image_data) = image_queue.pop() {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let last_update_value = *last_update.borrow();
                let new_last_update = draw_image_on_canvas(&canvas, image_data, last_update_value);
                *last_update.borrow_mut() = new_last_update;
            }
        }
    }) as Box<dyn FnMut()>)
}

#[derive(Properties, PartialEq, Clone)]
pub struct DisplayImageProps {
    pub image_queue: Arc<ImageQueue>,
}

#[function_component(DisplayImage)]
pub fn display_image(props: &DisplayImageProps) -> Html {
    let canvas_ref = use_node_ref();
    let last_update = Arc::new(RefCell::new(0.0));
    let image_queue = props.image_queue.clone();
    let canvas_ref_cloned = canvas_ref.clone();

    use_effect_with(image_queue.clone(), move |image_queue| {
        log::debug!("Initializing display image");
        let window: Window = web_sys::window().unwrap_throw();
        let last_update_cloned = last_update.clone();

        let callback = create_interval_callback(
            image_queue.clone(),
            canvas_ref_cloned,
            last_update_cloned.clone(),
        );

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
        <canvas id="display-image-canvas" ref={canvas_ref} />
    }
}
