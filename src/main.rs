use components::{card::Card, input_ref::InputComponent, nav_bar::NavBar};
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod components;
pub enum Msg {
    HoverIndex(usize),
    Submit,
}
pub struct App {
    refs: Vec<NodeRef>,
    focus_index: usize,
    title_error: String,
    content_error: String,
}
impl App {
    fn apply_focus(&self) {
        if let Some(input) = self.refs[self.focus_index].cast::<HtmlInputElement>() {
            input.focus().unwrap();
        }
    }
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            focus_index: 0,
            refs: vec![NodeRef::default(), NodeRef::default()],
            title_error: "".to_string(),
            content_error: "".to_string(),
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.apply_focus();
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HoverIndex(index) => {
                self.focus_index = index;
                self.apply_focus();
                false
            }
            Msg::Submit => {
                let title = &self.refs[0];
                let content = &self.refs[1];
                let title_value = title.cast::<HtmlInputElement>().unwrap().value();
                let content_value = content.cast::<HtmlInputElement>().unwrap().value();

                self.title_error.clear();
                self.content_error.clear();

                if !(title_value.contains('^') && title_value.contains('%')) {
                    self.title_error
                        .push_str("Invalid Title please do not add '^' or '%'.")
                }
                if content_value.len() < 8 {
                    self.content_error
                        .push_str("Content must be at least 50 characters long.")
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div class="container">
            <NavBar />
            <div class="main">
                    <div class="form-container">
                        <h1>{"Create your Note"}</h1>
                        <div class="input-container">
                            <label>{ "Title" }</label>
                            <InputComponent
                                input_ref={&self.refs[0]}
                                on_hover={ctx.link().callback(|_| Msg::HoverIndex(0))}
                                placeholder="Title"
                            />
                            <div class="error">{self.title_error.clone()}</div>
                        </div>
                        <div class="input-container">
                            <label>{ "Content" }</label>
                            <InputComponent
                                input_ref={&self.refs[1]}
                                on_hover={ctx.link().callback(|_| Msg::HoverIndex(1))}
                                placeholder="content"
                            />
                            <div class="error">{self.content_error.clone()}</div>
                        </div>
                        <button onclick={ctx.link().callback(|_| Msg::Submit)}>{"Create"}</button>
                    </div>
                </div>
            </div>
        }
    }
}

// #[function_component]
// fn App() -> Html {
//     let note = use_node_ref();

//     html! {
//         <div class={classes!("container")}>
//             <NavBar />
//             <div class={classes!("content")}>
//                 <div>
//                     <InputComponent
//                                 input_ref={note}
//                                 placeholder="password" />
//                 </div>
//                 <Card title={String::from("Test Title")} content={String::from("content test")} />
//             </div>
//         </div>
//     }
// }

fn main() {
    yew::Renderer::<App>::new().render();
}
