use yew::prelude::*;
use yew::Properties;

pub struct CustomButton;

impl Component for CustomButton {
    type Message = ();

    type Properties = ButtonProps;

    fn create(_: &Context<Self>) -> Self {
        return Self;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let overwrite_class = props.class.to_owned();
        let onclick = Callback::from(props.onclick.to_owned());
        let text = props.text.to_owned();
        let extra_class = props.extra_class.to_owned();

        html! {
            <button
            class = {
                let mut final_class = "generic-btn".to_string();

                if overwrite_class.trim().is_empty() == false{
                    final_class = overwrite_class;
                }

                if extra_class.trim().is_empty() == false{
                    final_class.push_str(&format!(" {}", extra_class));
                }

                final_class
            }

            {onclick}>

            {text}
            </button>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or("".to_string())]
    pub class: String,

    pub onclick: Callback<MouseEvent>,

    #[prop_or("".to_string())]
    pub text: String,

    #[prop_or("".to_string())]
    pub extra_class: String,
}
