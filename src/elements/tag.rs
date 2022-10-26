#![allow(clippy::redundant_closure_call)]

use yew::prelude::*;

use crate::Size;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TagProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "span".into())]
    pub tag: String,
    /// The click handler for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    /// Make this tag rounded.
    #[prop_or_default]
    pub rounded: bool,
    /// Turn this tag into a delete button.
    #[prop_or_default]
    pub delete: bool,
    /// The size for this component.
    #[prop_or_default]
    pub size: Option<Size>,
}

/// A small tag label to insert anywhere.
///
/// [https://bulma.io/documentation/elements/tag/](https://bulma.io/documentation/elements/tag/)
pub struct Tag;

impl Component for Tag {
    type Message = ();
    type Properties = TagProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("tag");
        classes.push(props.classes.clone());
        if props.rounded {
            classes.push("is-rounded");
        }
        if props.delete {
            classes.push("is-delete");
        }
        if let Some(size) = &props.size {
            classes.push(&size.to_string());
        }
        let tag = props.tag.clone();
        html! {
            <@{tag} class={classes} onclick={props.onclick.clone()}>
                {props.children.clone()}
            </@>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TagsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// Attach two tags together; this requires that this component wraps two `Tag` components.
    #[prop_or_default]
    pub has_addons: bool,
}

/// A container for a list of tags.
///
/// [https://bulma.io/documentation/elements/tag/](https://bulma.io/documentation/elements/tag/)
pub struct Tags;

impl Component for Tags {
    type Message = ();
    type Properties = TagsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("tags");
        classes.push(props.classes.clone());
        if props.has_addons {
            classes.push("has-addons");
        }
        html! {
            <div class={classes}>
                {props.children.clone()}
            </div>
        }
    }
}
