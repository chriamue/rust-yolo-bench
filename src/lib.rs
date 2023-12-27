use yew::prelude::*;

pub mod video;

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <video::Video />
        </div>
    }
}
