use yew::{prelude::*, virtual_dom::VList, virtual_dom::VNode, Properties};

pub struct Menu {
    props: Props,
}

pub enum ConnectIcon {
    Settings,
    Stats,
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
                <div class="w-full flex md:justify-end">
                  <div class="w-full flex md:w-32 justify-around">
                    { render_if(html!{icon(ConnectIcon::Stats, "/statistics")}, self.props.show_stats) }
                    { render_if(html!{icon(ConnectIcon::Settings, "/settings")}, self.props.show_settings) }
                  </div>
                </div>
            </div>
        }
    }
}
fn icon(i: ConnectIcon, dest: &str) -> VNode {
    html! {
      <a href={ yew::html::Href::from(dest)}> { icon_to_html(i) } </a>
    }
}
fn render_if(render: VNode, condition: bool) -> VNode {
    if condition {
        render
    } else {
        VNode::from(VList::new())
    }
}

fn icon_to_html(i: ConnectIcon) -> Html {
    match i {
        ConnectIcon::Settings => html!{<span class="material-icons">{"settings"}</span>},
        ConnectIcon::Stats    => html!{<span class="material-icons">{"bar_chart"}</span>},
    }
}
