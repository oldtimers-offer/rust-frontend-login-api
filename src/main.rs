use yew::prelude::*;
mod api;
mod components;
mod pages;
use crate::pages::login::Login;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Login />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
