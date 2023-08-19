use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {
    pub id: i32,
    pub title: String,
    pub content: String,
}

#[function_component]
pub fn Card(props: &CardProps) -> Html {
    html! {
        <div class={classes!("card")}>
            <h1>{props.title.clone()}</h1>
            <p>{props.content.clone()}</p>
        </div>
    }
}
