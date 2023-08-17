use components::card::Card;
use components::nav_bar::NavBar;
use yew::prelude::*;
mod components;
#[function_component]
fn App() -> Html {
    html! {
        <div class={classes!("container")}>
            <NavBar />
            <div class={classes!("content")}>
                <Card title={String::from("Test Title")} content={String::from("content test")} />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
