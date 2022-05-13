use derive_more::Display;
use yew::events::{Event, FocusEvent, MouseEvent};
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The size for all buttons within this group.
    #[prop_or_default]
    pub size: Option<ButtonGroupSize>,
}

/// A container for a group of buttons.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
pub struct Buttons;

impl Component for Buttons {
    type Message = ();
    type Properties = ButtonsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("buttons");
        classes.push(&props.classes);
        if let Some(size) = &props.size {
            classes.push(&size.to_string());
        }
        html! {
            <div class={classes}>
                {props.children.clone()}
            </div>
        }
    }
}

/// The 3 sizes available for a button group.
///
/// https://bulma.io/documentation/elements/button/#sizes
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "are-{}")]
pub enum ButtonGroupSize {
    #[display(fmt = "small")]
    Small,
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The click handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// A button element.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
pub struct Button;

impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("button");
        classes.push(&props.classes);
        if props.loading {
            classes.push("is-loading")
        }
        if props.r#static {
            classes.push("is-static")
        }
        html! {
            <button class={classes} onclick={props.onclick.clone()} disabled={props.disabled}>
                {props.children.clone()}
            </button>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "router")]
mod router {
    use super::*;
    use yew_router::components::Link;
    use yew_router::Routable;

    #[derive(Clone, Properties, PartialEq)]
    pub struct ButtonRouterProps<R>
    where
        R: Routable,
    {
        /// The Switched item representing the route.
        pub route: R,
        /// Html inside the component.
        #[prop_or_default]
        pub children: Children,
        /// Classes to be added to component.
        #[prop_or_default]
        pub classes: Option<Classes>,
        /// Render a loading spinner within this component.
        #[prop_or_default]
        pub loading: bool,
        /// Make this component static.
        #[prop_or_default]
        pub r#static: bool,
        /// Disable this component.
        #[prop_or_default]
        pub disabled: bool,
    }

    /// A Yew Router button element with Bulma styling.
    pub struct ButtonRouter<R>
    where
        R: Routable + 'static,
    {
        marker: std::marker::PhantomData<R>,
    }

    impl<R> Component for ButtonRouter<R>
    where
        R: Routable + 'static,
    {
        type Message = ();
        type Properties = ButtonRouterProps<R>;

        fn create(_ctx: &Context<Self>) -> Self {
            Self { marker: std::marker::PhantomData }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let props = ctx.props();
            let mut classes = Classes::from(&props.classes);
            if !classes.contains("button") {
                classes.push("button")
            }
            if props.loading {
                classes.push("is-loading");
            }
            html! {
                <Link<R>
                    to={props.route.clone()}
                    disabled={props.disabled}
                    classes={classes}
                    children={props.children.clone()}
                />
            }
        }
    }
}

#[cfg(feature = "router")]
pub use router::{ButtonRouter, ButtonRouterProps};

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonAnchorProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The `href` attribute value to use for this component.
    #[prop_or_default]
    pub href: String,
    /// The click handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// An optional `rel` for when this element is using the `a` tag.
    #[prop_or_default]
    pub rel: Option<String>,
    /// An optional `target` for when this element is using the `a` tag.
    #[prop_or_default]
    pub target: Option<String>,
}

/// An anchor element styled as a button.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
pub struct ButtonAnchor;

impl Component for ButtonAnchor {
    type Message = ();
    type Properties = ButtonAnchorProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("button");
        classes.push(&props.classes);
        if props.loading {
            classes.push("is-loading")
        }
        if props.r#static {
            classes.push("is-static")
        }
        html! {
            <a
                class={classes}
                onclick={props.onclick.clone()}
                href={props.href.clone()}
                rel={props.rel.clone().unwrap_or_default()}
                target={props.target.clone().unwrap_or_default()}
                disabled={props.disabled}
            >
                {props.children.clone()}
            </a>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonInputSubmitProps {
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The submit handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onsubmit: Callback<FocusEvent>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// An input element with `type="submit"` styled as a button.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
pub struct ButtonInputSubmit;

impl Component for ButtonInputSubmit {
    type Message = ();
    type Properties = ButtonInputSubmitProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("button");
        classes.push(&props.classes);
        if props.loading {
            classes.push("is-loading")
        }
        if props.r#static {
            classes.push("is-static")
        }
        html! {
            <input type="submit" class={classes} onsubmit={props.onsubmit.clone()} disabled={props.disabled}/>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonInputResetProps {
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The reset handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onreset: Callback<Event>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// An input element with `type="reset"` styled as a button.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
pub struct ButtonInputReset;

impl Component for ButtonInputReset {
    type Message = ();
    type Properties = ButtonInputResetProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("button");
        classes.push(&props.classes);
        if props.loading {
            classes.push("is-loading")
        }
        if props.r#static {
            classes.push("is-static")
        }
        html! {
            <input type="reset" class={classes} onreset={props.onreset.clone()} disabled={props.disabled}/>
        }
    }
}
