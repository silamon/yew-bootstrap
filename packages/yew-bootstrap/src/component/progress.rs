use yew::prelude::*;

use crate::util::Color;

/// # Progress component
/// [Progress] is used to group several [crate::component::ProgressBar] instances together.
/// 
/// 
/// See [ProgressProps] and [ProgressBarProps] for a listing of properties
/// 
/// ## Example
/// Example of a simple progress:
/// 
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::Progress;
/// use yew_bootstrap::component::ProgressBar;
/// use yew_bootstrap::util::Color;
/// fn test() -> Html {
///     html!{
///         <Progress>
///             <ProgressBar style={Color::Success} min_value=0 max_value=10 now_value=3 />
///         </Progress>
///     }
/// }
/// ```
pub struct Progress {}

/// Properties for [Progress]
#[derive(Properties, Clone, PartialEq)]
pub struct ProgressProps {
    /// Children for the group (ProgressBar instances)
    #[prop_or_default]
    pub children: Children,
}

impl Component for Progress {
    type Message = ();
    type Properties = ProgressProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::new();
        classes.push("progress");
        
        html! {
            <div
                class="progress"
            >
                { for props.children.iter() }
            </div>
        }
    }
}

pub struct ProgressBar {}

/// # Properties of [ProgressBar]
#[derive(Properties, Clone, PartialEq)]
pub struct ProgressBarProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// Color style, default [Color::Primary]
    #[prop_or(Color::Primary)]
    pub style: Color,

    /// Optional text placed in the progress bar
    #[prop_or_default]
    pub text: String,

    /// Enable the striped effect on the progress bar
    #[prop_or_default]
    pub is_striped: bool,

    /// Enable the animation effect on the progress bar
    #[prop_or_default]
    pub is_animated: bool,

    /// The minimum value of the progress bar
    #[prop_or_default]
    pub min_value: u8,

    /// The maximum value of the progress bar
    #[prop_or_default]
    pub max_value: u8,

    /// The now or current value of the progress bar
    #[prop_or_default]
    pub now_value: u8,
}

impl Component for ProgressBar {
    type Message = ();
    type Properties = ProgressBarProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::new();
        classes.push("progress-bar");
        classes.push(format!("bg-{}", props.style));
        if props.is_striped {            
            classes.push("progress-bar-striped");
        }
        if props.is_animated {            
            classes.push("progress-bar-animated");
        }
        classes.push(props.class.clone());

        let percentage = (props.now_value as f32 / (props.max_value as f32 - props.min_value as f32)) * 100.0; 
     
        html! {
            <div 
                class={classes} 
                role="progressbar" 
                style={format!("width: {}%", percentage.to_string())} 
                aria-valuenow={ props.now_value.to_string() } 
                aria-valuemin={ props.min_value.to_string() } 
                aria-valuemax={ props.max_value.to_string() }>
                { &props.text }
            </div>
        }
    }
}