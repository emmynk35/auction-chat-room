use yew::prelude::*;

struct Chat {
    link: ComponentLink<Self>,
}

enum Msg {

}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Chat {

        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {

        }
    }

    fn view(&self) -> Html {

        html! {

        }
    }
}