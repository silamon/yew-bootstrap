use yew::prelude::*;

use crate::util::Color;

/// # Card
/// Card, parent of [CardHeader], [CardBody] and [CardFooter].
/// 
/// See [CardProps] for a listing of properties
/// 
/// ## Example
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::{Card, CardHeader, CardBody, CardFooter, Button};
/// use yew_bootstrap::util::Color;
/// fn test() -> Html {
///     html!{
///         <Card>
///             <CardHeader text="Header" />
///             <CardBody>
///                 <p>{"Card body text goes here."}</p>
///             </CardBody>
///             <CardFooter>
///                 <Button style={ Color::Secondary }>{ "Close" }</Button>
///                 <Button style={ Color::Primary }>{ "Save changes" }</Button>
///             </CardFooter>
///         </Card>
///     }
/// }
/// ```
pub struct Card { }

/// # Header for a [Card] dialog
/// See [CardHeaderProps] for a listing of properties
pub struct CardHeader { }

/// # Body for a [Card] dialog
/// See [CardBodyProps] for a listing of properties
pub struct CardBody { }

/// # Footer for a [Card] dialog
/// See [CardFooterProps] for a listing of properties
pub struct CardFooter { }

/// Properties for [CardFooter]
#[derive(Properties, Clone, PartialEq)]
pub struct CardFooterProps {
    /// Text for the card
    #[prop_or_default]
    pub text: String,
   
    /// Children for the card footer
    #[prop_or_default]
    pub children: Children
}

impl Component for CardFooter {
    type Message = ();
    type Properties = CardFooterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html! {
            <div class="card-footer">
                { &props.text }
                { for props.children.iter() }
            </div>
        }
    }
}

/// Properties for [CardHeader]
#[derive(Properties, Clone, PartialEq)]
pub struct CardHeaderProps {
    /// Text for the card
    #[prop_or_default]
    pub text: String,

    /// Children for the card header
    #[prop_or_default]
    pub children: Children
}

impl Component for CardHeader {
    type Message = ();
    type Properties = CardHeaderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html! {
            <div class="card-header">
                { &props.text }
                { for props.children.iter() }
            </div>
        }
    }
}

/// Properties for [CardBody]
#[derive(Properties, Clone, PartialEq)]
pub struct CardBodyProps {
    #[prop_or_default]
    pub children: Children
}

impl Component for CardBody {
    type Message = ();
    type Properties = CardBodyProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html! {
            <div class="card-body">
                { for props.children.iter() }
            </div>
        }
    }
}

/// Properties for Card
#[derive(Properties, Clone, PartialEq)]
pub struct CardProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// card body, typically [CardHeader], [CardBody] or [CardFooter]
    #[prop_or_default]
    pub children: Children,
    
    /// Color style, default [Color::Primary]
    #[prop_or(Color::Primary)]
    pub style: Color,
}

impl Component for Card {
    type Message = ();
    type Properties = CardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::new();
        classes.push("card");
        classes.push(format!("bg-{}", props.style));
        classes.push(props.class.clone());

        html! {
            <div
                class={classes} 
            >
                { for props.children.iter() }
            </div>
        }
    }
}