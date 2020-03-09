use yew::prelude::*;

struct PriceTable {

}

impl Component for PriceTable {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        PriceTable {

        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
       unimplemented!()
    }

    fn view(&self) -> Html {
        html! {
            <div class="ptable">
            <table>
                <tr>
                    <th>Item</th>
                    <th>Min Price</th>
                    <th>Ave Price</th>
                </tr>
                <tr>
                    <th>"Example Item"</th>
                    <th>"30"</th>
                    <th>"50"</th>
                </tr>
                <tr>
                    <th>"Example Item 2"</th>
                    <th>"50"</th>
                    <th>"5800"</th>
                </tr>
            </table>
            </div>
        }
    }
}