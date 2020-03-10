use yew::prelude::*;
use yew::components::Select;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Clone, Debug, Display, EnumString, EnumIter, PartialEq)]
enum Item {
    BronzeSword,
    IronPlatebody,
}

struct Input {
    link: ComponentLink<Self>,
    wtbuy: bool,
    price: usize,
    item: Option<Item>, 
}

enum Msg {
    changedBuySell(bool),
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
            wtbuy: True,
            price: 500,
            item: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::changedBuySell => {
                
            }
            Msg::choseItem(item) => {
                self.item = Some(item);
            }
            Msg::changedPrice = > {

            }
            Msg::hitEnter = > {

            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div>
                    <Select<Item>
                    selected=self.item.clone()
                    options=Item::iter().collect::<Vec<_>>()
                    onchange=self.link.callback(Msg::choseItem) />
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
// }