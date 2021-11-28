use yew::prelude::*;
use yew_bootstrap::component::*;
use yew_bootstrap::init_cdn;
use yew_bootstrap::util::*;

enum Msg {
    AddOne,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        /*
        <div>
            <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
            <p>{ self.value }</p>
        </div>
        */
        html! {
            <>
                {init_cdn()}
                <div>
                    <h1>{"Buttons"}</h1>
                    <Button style=Color::Primary>{"Primary"}</Button>
                    <Button style=Color::Secondary>{"Secondary"}</Button>
                    <Button style=Color::Success>{"Success"}</Button>
                    <Button style=Color::Danger>{"Danger"}</Button>
                    <Button style=Color::Warning>{"Warning"}</Button>
                    <Button style=Color::Info>{"Info"}</Button>
                    <Button style=Color::Light>{"Light"}</Button>
                    <Button style=Color::Dark>{"Dark"}</Button>
                    <Button style=Color::Link>{"Link"}</Button>

                    <h2>{"Outline buttons"}</h2>
                    <Button style=Color::Primary outline=true>{"Primary"}</Button>
                    <Button style=Color::Secondary outline=true>{"Secondary"}</Button>
                    <Button style=Color::Success outline=true>{"Success"}</Button>
                    <Button style=Color::Danger outline=true>{"Danger"}</Button>
                    <Button style=Color::Warning outline=true>{"Warning"}</Button>
                    <Button style=Color::Info outline=true>{"Info"}</Button>
                    <Button style=Color::Light outline=true>{"Light"}</Button>
                    <Button style=Color::Dark outline=true>{"Dark"}</Button>
                    <Button style=Color::Link outline=true text="Link2" />
                </div>
            </>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

fn main() {
    yew::start_app::<Model>();
}