use yew::{prelude::*, virtual_dom::VList, virtual_dom::VNode, Properties};

pub struct MenuButton {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub text: String,
    pub dest: String,
}

pub enum Msg {}

impl Component for MenuButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <a href={ yew::html::Href::from(self.props.dest.as_str()) } class="">
              { &self.props.text }
            </a>
        }
    }
}
