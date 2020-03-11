#![recursion_limit="512"]
use yew::prelude::*;
use yew::components::Select;
use yew::html::InputData;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, EnumString, EnumIter, PartialEq, Eq, Hash)]
enum Item {
    Sword,
    Helmet,
    Bow,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Bow => write!(f, "Bow"),
            Item::Helmet => write!(f, "Helmet"),
            Item::Sword => write!(f, "Sword"),
        }
    }
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
    wtb_max_prices: HashMap<Item, usize>,
    wts_min_prices: HashMap<Item, usize>,
    wtb_history: Vec<(Item, usize)>,
    wts_history: Vec<(Item, usize)>,
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
            wtb_max_prices: HashMap::new(),
            wts_min_prices: HashMap::new(),
            wtb_history: Vec::new(),
            wts_history: Vec::new(),
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
                self.price = price;
            }
            Msg::HitEnter => {
                if let Some(item) = &self.item {
                    if &self.action == &Action::WTB {
                        if let Some(cost) = &self.wtb_max_prices.get(&item) {
                            if &self.price > cost {
                                self.wtb_max_prices.insert(item.clone(), self.price);
                            }
                        } else {
                            self.wtb_max_prices.insert(item.clone(), self.price);
                        }
                        self.wtb_history.push((item.clone(), self.price));
                    } else {
                        if let Some(cost) = &self.wts_min_prices.get(&item) {
                            if &self.price < cost {
                                self.wts_min_prices.insert(item.clone(), self.price);
                            }
                        } else {
                            self.wts_min_prices.insert(item.clone(), self.price);
                        }
                        self.wts_history.push((item.clone(), self.price));
                    }
                    self.price = 500;
                    self.item = None;
                }
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Select<Action>
                selected=self.action.clone()
                options=Action::iter().collect::<Vec<_>>()
                onchange=self.link.callback(Msg::ChangedAction) />

                <Select<Item>
                selected=self.item.clone()
                options=Item::iter().collect::<Vec<_>>()
                onchange=self.link.callback(Msg::ChoseItem) />

                <input value=&self.price type="number" min="1" max="2147483647" oninput=self.link.callback(|e: InputData| Msg::ChangedPrice(e.value.parse::<usize>().unwrap()))/>
                <button onclick=self.link.callback(|_| Msg::HitEnter)>{ "Post Item" }</button>

                <div>
                    <p>{ "Buyers" }</p>
                    <ul>
                        { for self.wtb_history.iter().map(|e| self.show_items(e)) }
                    </ul>
                </div>

                <div>
                    <p>{ "Sellers" }</p>
                    <ul>
                        { for self.wts_history.iter().map(|e| self.show_items(e)) }
                    </ul>
                </div>

                <div>
                    <p>{ "Maximum Prices for Buy" }</p>
                    <ul>
                        { for self.wtb_max_prices.iter().map(|e| self.show_top_deals(e)) }
                    </ul>
                </div>

                <div>
                    <p>{ "Minimum Prices for Sale" }</p>
                    <ul>
                        { for self.wts_min_prices.iter().map(|e| self.show_top_deals(e)) }
                    </ul>
                </div>
            </>
        }
    }
}

impl Input {
    fn show_items(&self, (item, price): &(Item, usize)) -> Html {
        html! {
            <li>
                { format!("{:?}: {:?}", item, price) }
            </li>
        }
    }
    fn show_top_deals(&self, (item, price): (&Item, &usize)) -> Html {
        html! {
            <li>
                { format!("{:?}: {:?}", item, price) }
            </li>
        }
    }
}

fn main() {
    yew::start_app::<Input>();
}