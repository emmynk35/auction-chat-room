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
    changedAction(Action),
    choseItem(Item),
    changedPrice(usize),
    hitEnter,
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
            Msg::changedAction(action) => {
                self.action = action;
            }
            Msg::choseItem(item) => {
                self.item = Some(item);
            }
            Msg::changedPrice(price) = > {
                self.price = price
            }
            Msg::hitEnter = > {
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
                    onchange=self.link.callback(Msg::changedAction) />

                    <Select<Item>
                    selected=self.item.clone()
                    options=Item::iter().collect::<Vec<_>>()
                    onchange=self.link.callback(Msg::choseItem) />

                    <input type="number" min="1" max="2147483647" oninput=self.link.callback(|e: InputData| Msg::changedPrice(e.value))>

                    <button onclick=self.link.callback(|_| Msg::hitEnter)>{ "Post Item" }</button>
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