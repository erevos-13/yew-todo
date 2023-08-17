use yew::prelude::*;
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ButtonProps {
    pub name: String,
    pub callback: Callback<String>,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let onclick = Callback::from(move |_| {
        let greeting = String::from("Hi there");
    });

    html! {
        <button {onclick}>{props.name.clone()}</button>
    }
}
