use yew::prelude::*;

#[function_component(FooterComponent)]
pub fn footer_component() -> Html {
    html! {
        <div class="footer">
            {"Welcome to the footer component."}
        </div>
    }
}
