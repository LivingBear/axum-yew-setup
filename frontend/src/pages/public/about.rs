use yew::prelude::*;

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    html! {
        <>
        <header class={classes!("w-100", "black-bg")}>
            <h1>
                {"About Page"}
            </h1>        
        </header>
        </>
    }
}
