#![allow(clippy::redundant_closure_call)]

use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MediaProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// A UI element for repeatable and nestable content.
///
/// [https://bulma.io/documentation/layout/media-object/](https://bulma.io/documentation/layout/media-object/)
pub struct Media; 

impl Component for Media {
    type Message = ();
    type Properties = MediaProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("media");
        classes.push(&props.classes);
        html! {
            <@{props.tag.clone()} class={classes}>
                {props.children.clone()}
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MediaLeftProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// Elements to be grouped to the left of the media container.
///
/// [https://bulma.io/documentation/layout/media-object/](https://bulma.io/documentation/layout/media-object/)
pub struct MediaLeft; 

impl Component for MediaLeft {
    type Message = ();
    type Properties = MediaLeftProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("media-left");
        classes.push(&props.classes);
        html! {
            <@{props.tag.clone()} class={classes}>
                {props.children.clone()}
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MediaRightProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// Elements to be grouped to the right of the media container.
///
/// [https://bulma.io/documentation/layout/media-object/](https://bulma.io/documentation/layout/media-object/)
pub struct MediaRight; 

impl Component for MediaRight {
    type Message = ();
    type Properties = MediaRightProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("media-right");
        classes.push(&props.classes);
        html! {
            <@{props.tag.clone()} class={classes}>
                {props.children.clone()}
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MediaContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// Elements to be grouped as the center body of the media container.
///
/// [https://bulma.io/documentation/layout/media-object/](https://bulma.io/documentation/layout/media-object/)
pub struct MediaContent; 

impl Component for MediaContent {
    type Message = ();
    type Properties = MediaContentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("media-content");
        classes.push(&props.classes);
        html! {
            <@{props.tag.clone()} class={classes}>
                {props.children.clone()}
            </@>
        }
    }
}
