use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: String,
    pub on_change: Callback<String>,
    pub input_type: InputType,
}
#[derive(Clone, PartialEq)]
pub enum InputType {
    Username,
    Password,
    Email,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    web_sys::console::log_1(&target.value().into());
    target.value()
}

/// Controlled Text Input Component
#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props { value, on_change, input_type } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(get_value_from_input_event(input_event));
    });

    let input_element = match input_type {
        InputType::Username => html! {
            <input type="text" {value} {oninput} pattern="[a-zA-Z0-9_-]{3,16}" title="3-16 characters, letters, numbers, underscores, or hyphens only" />
        },
        InputType::Password => html! {
            <input type="password" {value} {oninput} pattern=".{8,32}" title="8-32 characters" />
        },
        InputType::Email => html! {
            <input type="email" {value} {oninput} />
        },
    };

    html! {
        { input_element }
    }

}
