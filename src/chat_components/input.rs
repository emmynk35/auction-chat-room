use yew::prelude::*;
use yew::components::Select;
use yew::html::InputData;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Clone, Debug, Display, EnumString, EnumIter, PartialEq)]
enum Item {
    BronzeSword,
    IronPlatebody,
}

#[derive(Clone, Debug, Display, EnumString, EnumIter, PartialEq)]
enum Action {
    WTB,
    WTS,
}

struct Input {
    link: ComponentLink<Self>,
    action: Action,
    price: usize,
    item: Option<Item>, 
}

enum Msg {
    ChangedAction(Action),
    ChoseItem(Item),
    ChangedPrice(usize),
    HitEnter,
}

impl Component for Input {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Input {
            link,
            action: Action::WTB,
            price: 500,
            item: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangedAction(action) => {
                self.action = action;
            }
            Msg::ChoseItem(item) => {
                self.item = Some(item);
            }
            Msg::ChangedPrice(price) => {
                self.price = price
            }
            Msg::HitEnter => {
                unimplemented!
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div>
                    <Select<Action>
                    selected=self.action.clone()
                    options=Action::iter().collect::<Vec<_>>()
                    onchange=self.link.callback(Msg::ChangedAction) />

                    <Select<Item>
                    selected=self.item.clone()
                    options=Item::iter().collect::<Vec<_>>()
                    onchange=self.link.callback(Msg::ChoseItem) />

                    <input type="number" min="1" max="2147483647" oninput=self.link.callback(|e: InputData| Msg::ChangedPrice(e.value.parse::<usize>().unwrap()))>

                    <button onclick=self.link.callback(|_| Msg::HitEnter)>{ "Post Item" }</button>
                </div>
            </>
        }
    }
}

// impl Input {
//     fn show_cur_item(&self) {
//         if let Some(item) = self.item.as_ref() {
//             match item {
//                 Item::BronzeSword => html! { <p>{ "Bronze Sword" }</p> },
//                 Item::IronPlatebody => html! { <p>{ "Iron Axe" }</p>},
//             }
//         } else {
//             html! {
//                 <p>{ "Nothing rn" }</p>
//             }
//         }
//     }

//     fn submit(&self) {

//     }
// }