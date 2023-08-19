use components::{
    card::Card, input_ref::InputComponent, nav_bar::NavBar, textarea::TextareaComponent,
};
use web_sys::{console::log, HtmlInputElement};
use yew::prelude::*;

mod components;
pub enum Msg {
    HoverIndex(usize),
    Submit,
}

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Note {
    id: i32,
    title: String,
    content: String,
}
pub struct App {
    refs: Vec<NodeRef>,
    focus_index: usize,
    title_error: String,
    content_error: String,
    notes: Vec<Note>,
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
            notes: vec![],
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
                let mut correct_content = true;
                let title = &self.refs[0];
                let content = &self.refs[1];
                let title_value = title.cast::<HtmlInputElement>().unwrap().value();
                let content_value = content.cast::<HtmlInputElement>().unwrap().value();

                self.title_error.clear();
                self.content_error.clear();

                if title_value.contains('^') && title_value.contains('%') {
                    self.title_error
                        .push_str("Invalid Title please do not add '^' or '%'.");
                    correct_content = false;
                }
                if content_value.len() < 8 {
                    self.content_error
                        .push_str("Content must be at least 50 characters long.");
                    correct_content = false;
                }
                if correct_content {
                    self.notes.push(Note {
                        id: self.focus_index as i32,
                        title: title_value,
                        content: content_value,
                    });
                    title.cast::<HtmlInputElement>().unwrap().set_value("");
                    content.cast::<HtmlInputElement>().unwrap().set_value("");
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_click_card: Callback<i32> = Callback::from(move |note_id: i32| {
            let greeting = format!("Hey, {}!", note_id);
            web_sys::console::log_1(&greeting.into()); // if uncommented will print
        });

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
                            <TextareaComponent
                                textarea_ref={&self.refs[1]}
                                on_hover={ctx.link().callback(|_| Msg::HoverIndex(1))}
                                placeholder="Content"
                            />
                            <div class="error">{self.content_error.clone()}</div>
                        </div>
                        <button onclick={ctx.link().callback(|_| Msg::Submit)}>{"Create"}</button>
                    </div>
            </div>
            <div class="notes-container">
                {
                self.notes.clone().into_iter().map(|note| {
                    html! {
                    <div  class="item">
                        <Card  id={note.id} title={note.title} content={note.content} />
                    </div>
                    }
                }).collect::<Html>()

            }
            </div>

        </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
