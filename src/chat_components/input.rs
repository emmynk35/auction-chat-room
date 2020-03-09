use yew::prelude::*;

enum Item {
    None,
    Bronze_Sword,
    Iron_Platebody,
}

struct Input {
    link: ComponentLink<Self>,
    wtbuy: bool,
    price: usize,
    item: Item, 
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
            Msg::choseItem => {

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
            <div>
                <select onchange=self.link.callback(function: F)>
                    <option value="none">{ "Select an Item" }</option>
                    <option value="sword">{ "Bronze Sword" }</option>
                    <option value="platebody">{ "Iron Platebody" }</option>
                </select>
            </div>
        }
    }
}