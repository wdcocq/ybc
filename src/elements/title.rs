#![allow(clippy::redundant_closure_call)]

use derive_more::Display;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TitleProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "h3".into())]
    pub tag: String,
    /// Maintain the normal spacing between titles and subtitles.
    #[prop_or_default]
    pub is_spaced: bool,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<HeaderSize>,
}

/// A simple heading to add depth to your page.
///
/// [https://bulma.io/documentation/elements/title/](https://bulma.io/documentation/elements/title/)
pub struct Title;

impl Component for Title {
    type Message = ();
    type Properties = TitleProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("title");
        classes.push(props.classes.clone());
        if let Some(size) = &props.size {
            classes.push(&size.to_string());
        }
        if props.is_spaced {
            classes.push("is-spaced");
        }
        let tag = props.tag.clone();
        html! {
            <@{tag} class={classes}>
                {props.children.clone()}
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SubtitleProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "h3".into())]
    pub tag: String,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<HeaderSize>,
}

/// A simple heading to add depth to your page.
///
/// [https://bulma.io/documentation/elements/title/](https://bulma.io/documentation/elements/title/)
pub struct Subtitle;

impl Component for Subtitle {
    type Message = ();
    type Properties = SubtitleProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("subtitle");
        classes.push(props.classes.clone());
        if let Some(size) = &props.size {
            classes.push(&size.to_string());
        }
        let tag = props.tag.clone();
        html! {
            <@{tag} class={classes}>
                {props.children.clone()}
            </@>
        }
    }
}

/// The six sizes available for titles & subtitles.
///
/// https://bulma.io/documentation/elements/title/#sizes
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum HeaderSize {
    #[display(fmt = "1")]
    Is1,
    #[display(fmt = "2")]
    Is2,
    #[display(fmt = "3")]
    Is3,
    #[display(fmt = "4")]
    Is4,
    #[display(fmt = "5")]
    Is5,
    #[display(fmt = "6")]
    Is6,
}
