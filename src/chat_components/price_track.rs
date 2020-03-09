use yew::prelude::*
use price_table::PriceTable;

pub struct PriceTracker {
    clicked = bool,
    onclick: Callback<ClickEvent>,
}

enum Msg {
    Click
}

impl Component for PriceTracker {
    type Message = ();
    type Properties = ();
    
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        PriceTracker {
            clicked = false,
            onclick: link.callback(|_| Msg::Click),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true;
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="pt">
                <div class = "sbar">
                    <input type="text" placeholder="Search item">
                </div>
                <PriceTable />
                <div class="dm">
                    <h2>"Send DM"</h2>
                    <form>
                        <label for="usname">Recipient Username: </label>
                        <input type="text" id="usname" name="usname"><br><br>
                        <label for="msg">Message: </label>
                        <input type="text" id="msg" name="msg"><br><br>
                        <input type="submit" value="Submit">
                    </form>>
                </div>
            </div>
        }
    } 
}