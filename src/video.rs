use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys;
use web_sys::{HtmlVideoElement, MediaStreamConstraints};
use yew::prelude::*;

async fn get_user_media() -> Result<HtmlVideoElement, JsValue> {
    let window = web_sys::window().unwrap_throw();
    let navigator = window.navigator();
    let media_devices = navigator.media_devices()?;
    let mut constraints = MediaStreamConstraints::new();
    constraints.video(&JsValue::from(true));

    let media_stream_promise = media_devices.get_user_media_with_constraints(&constraints)?;
    let media_stream = JsFuture::from(media_stream_promise).await?;

    let document = window.document().unwrap_throw();
    let video = document.create_element("video")?;
    video.set_attribute("autoplay", "true")?;
    video.set_attribute("width", "640")?;
    video.set_attribute("height", "480")?;
    video.set_attribute("id", "video")?;
    // video.set_attribute("style", "display: none").unwrap();
    document.body().unwrap_throw().append_child(&video)?;
    let video = video.dyn_into::<HtmlVideoElement>()?;
    video.set_src_object(Some(&media_stream.unchecked_into()));

    Ok(video)
}

async fn play_video(video: HtmlVideoElement) {
    match video.play() {
        Ok(stream) => {
            let stream = stream.dyn_into::<js_sys::Promise>().unwrap();
            if let Err(e) = JsFuture::from(stream).await {
                log::error!("Error playing the video: {:?}", e);
            }
        }
        Err(e) => log::error!("Error starting the video: {:?}", e),
    }
}

fn init_webcam() {
    wasm_bindgen_futures::spawn_local(async {
        match get_user_media().await {
            Ok(video) => {
                log::info!("starting Video");
                play_video(video).await
            }
            Err(err) => log::error!("Error accessing the webcam: {:?}", err),
        }
    });
}

#[function_component]
pub fn Video() -> Html {
    let video_ref = use_node_ref();

    use_effect(move || {
        init_webcam();
        || {}
    });

    html! {
        <div>
            <video ref={video_ref} autoplay=true />
        </div>
    }
}
