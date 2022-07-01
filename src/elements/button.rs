use crate::Size;
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
#[function_component(Buttons)]
pub fn buttons(ButtonsProps { children, classes, size }: &ButtonsProps) -> Html {
    let classes = classes!(classes, "buttons", size);

    html! {
        <div class={classes}>
            {children.clone()}
        </div>
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

impl From<ButtonGroupSize> for Classes {
    fn from(size: ButtonGroupSize) -> Self {
        size.to_string().into()
    }
}

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
    /// The size of the button
    #[prop_or_default]
    pub size: Option<Size>,
    /// Render a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// Rounded corners
    #[prop_or_default]
    pub rounded: bool,
    /// Invert color scheme
    #[prop_or_default]
    pub inverted: bool,
    /// Provide an outline
    #[prop_or_default]
    pub outline: bool,
}

/// A button element.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)

#[function_component(Button)]
pub fn button(
    ButtonProps {
        children,
        classes,
        onclick,
        size,
        loading,
        r#static,
        disabled,
        rounded,
        inverted,
        outline,
    }: &ButtonProps,
) -> Html {
    let classes = classes!(
        classes,
        "button",
        size,
        loading.then_some("is-loading"),
        r#static.then_some("is-static"),
        inverted.then_some("is-inverted"),
        outline.then_some("is-outline"),
        rounded.then_some("is-rounded")
    );

    html! {
        <button class={classes} {onclick} disabled={*disabled}>
            {children.clone()}
        </button>
    }
}

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
    /// Rounded corners
    #[prop_or_default]
    pub rounded: bool,
    /// Invert color scheme
    #[prop_or_default]
    pub inverted: bool,
    /// Provide an outline
    #[prop_or_default]
    pub outline: bool,
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
#[function_component(ButtonAnchor)]
pub fn button_anchor(
    ButtonAnchorProps {
        children,
        classes,
        href,
        onclick,
        loading,
        r#static,
        disabled,
        rounded,
        inverted,
        outline,
        rel,
        target,
    }: &ButtonAnchorProps,
) -> Html {
    let classes = classes!(
        classes,
        "button",
        loading.then_some("is-loading"),
        r#static.then_some("is-static"),
        inverted.then_some("is-inverted"),
        outline.then_some("is-outline"),
        rounded.then_some("is-rounded")
    );
    html! {
        <a
            class={classes}
            {onclick}
            href={href.clone()}
            rel={rel.clone().unwrap_or_default()}
            target={target.clone().unwrap_or_default()}
            disabled={*disabled}
        >
            {children.clone()}
        </a>
    }
}

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
    /// Rounded corners
    #[prop_or_default]
    pub rounded: bool,
    /// Invert color scheme
    #[prop_or_default]
    pub inverted: bool,
    /// Provide an outline
    #[prop_or_default]
    pub outline: bool,
}

/// An input element with `type="submit"` styled as a button.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
#[function_component(ButtonInputSubmit)]
pub fn button_input(
    ButtonInputSubmitProps {
        classes,
        onsubmit,
        loading,
        r#static,
        disabled,
        rounded,
        inverted,
        outline,
    }: &ButtonInputSubmitProps,
) -> Html {
    let classes = classes!(
        classes,
        "button",
        loading.then_some("is-loading"),
        r#static.then_some("is-static"),
        inverted.then_some("is-inverted"),
        outline.then_some("is-outline"),
        rounded.then_some("is-rounded")
    );

    html! {
        <input type="submit" class={classes} {onsubmit} disabled={*disabled}/>
    }
}

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
    /// Rounded corners
    #[prop_or_default]
    pub rounded: bool,
    /// Invert color scheme
    #[prop_or_default]
    pub inverted: bool,
    /// Provide an outline
    #[prop_or_default]
    pub outline: bool,
}

/// An input element with `type="reset"` styled as a button.
///
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
#[function_component(ButtonInputReset)]
pub fn button_input_reset(
    ButtonInputResetProps {
        classes,
        onreset,
        loading,
        r#static,
        disabled,
        rounded,
        inverted,
        outline,
    }: &ButtonInputResetProps,
) -> Html {
    let classes = classes!(
        classes,
        "button",
        loading.then_some("is-loading"),
        r#static.then_some("is-static"),
        inverted.then_some("is-inverted"),
        outline.then_some("is-outline"),
        rounded.then_some("is-rounded")
    );

    html! {
        <input type="reset" class={classes} {onreset} disabled={*disabled}/>
    }
}
