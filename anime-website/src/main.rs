pub use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <html>
        <body>
            <iframe style="border:0" src="https://www.reddit.com/" width="750" height="300"></iframe>
        </body>

        </html>
                }
    }
}

fn main() {
    yew::start_app::<Model>();
}
