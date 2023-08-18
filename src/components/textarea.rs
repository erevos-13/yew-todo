use yew::prelude::*;

pub enum Msg {
    Hover,
}
#[derive(Properties, PartialEq)]
pub struct TextareaProps {
    pub on_hover: Callback<()>,
    pub placeholder: AttrValue,
    pub textarea_ref: NodeRef,
}
pub struct TextareaComponent;

impl Component for TextareaComponent {
    type Message = Msg;
    type Properties = TextareaProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Hover => {
                ctx.props().on_hover.emit(());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let placeholder = ctx.props().placeholder.clone();
        html! {
            <textarea
                rows="4" cols="60"
                ref={&ctx.props().textarea_ref}
                class="textarea-component"
                placeholder={placeholder}
                onmouseover={ctx.link().callback(|_| Msg::Hover)}
            ></textarea>
        }
    }
}
