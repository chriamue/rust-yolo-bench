use web_sys::HtmlSelectElement;
use yew::prelude::*;

use crate::Model;

#[derive(Properties, PartialEq, Clone)]
pub struct ModelSelectionProps {
    pub on_select: Callback<Model>,
}

#[function_component(ModelSelection)]
pub fn model_selection(props: &ModelSelectionProps) -> Html {
    let on_change = {
        let on_select = props.on_select.clone();
        Callback::from(move |event: Event| {
            let target: HtmlSelectElement = event.target_unchecked_into();
            let value = target.value();
            let model = match value.as_str() {
                #[cfg(feature = "candle")]
                "Candle" => Model::Candle,
                #[cfg(feature = "tract")]
                "Tract" => Model::Tract,
                _ => Model::None,
            };
            on_select.emit(model);
        })
    };

    html! {
        <select onchange={on_change}>
            <option value="None" selected=true>{"None"}</option>
            <option value="Candle">{"Candle"}</option>
            <option value="Tract">{"Tract"}</option>
        </select>
    }
}
