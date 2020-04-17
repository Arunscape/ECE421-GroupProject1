use yew::{prelude::*, Properties};

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

    fn view(&self) -> Html {
        let c = "bg-blue-500 hover:bg-blue-400 text-white font-bold py-2 px-4 border-b-4 border-blue-700 hover:border-blue-500 rounded text-center my-1 mx-1";
        html! {
            <a href={ yew::html::Href::from(self.props.dest.as_str()) } class=c >
              { &self.props.text }
            </a>
        }
    }
}
