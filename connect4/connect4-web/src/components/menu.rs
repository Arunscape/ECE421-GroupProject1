use yew::{prelude::*, virtual_dom::VList, virtual_dom::VNode, Properties};

pub struct Menu {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub topbar: String,
    pub title: String,
    pub show_stats: bool,
    pub show_settings: bool,
    pub children: Children,
}

pub enum Msg {}

impl Component for Menu {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="h-full flex flex-col items-center justify-between">
                <div class="w-full text-left"> { &self.props.topbar } </div>
                <h1 class="font-comic text-6xl">{ &self.props.title }</h1>
                <div>
                    { self.props.children.render() }
                </div>
                <div>
                  { render_if(html!{icon()}, self.props.show_stats) }
                  { render_if(html!{icon()}, self.props.show_settings) }
                </div>
            </div>
        }
    }
}
fn icon() -> VNode {
    html! {
        <p> { "#" } </p>
    }
}
fn render_if(render: VNode, condition: bool) -> VNode {
    if condition {
        render
    } else {
        VNode::from(VList::new())
    }
}
