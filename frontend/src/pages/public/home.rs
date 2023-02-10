use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <>
        {"Hi from home"}
        <img src="./public/logicalfallacy.jpg" alt="rust image"/> 
        <div class={classes!("container")}>{"Container div"}</div>
        </>
    }

}
