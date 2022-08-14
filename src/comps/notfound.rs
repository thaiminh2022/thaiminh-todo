use yew::{html, prelude::function_component};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class = "nf">
            <h1>{"404: WHAT THE FUCK IS GOING ON"}</h1>
        </div>
    }
}
