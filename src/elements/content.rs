#![allow(clippy::redundant_closure_call)]

use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// A single component to wrap WYSIWYG generated content, where only HTML tags are available.
///
/// [https://bulma.io/documentation/elements/content/](https://bulma.io/documentation/elements/content/)
pub struct Content;

impl Component for Content {
    type Message = ();
    type Properties = ContentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("content");
        classes.push(&props.classes);
        let tag = props.tag.clone();
        html! {
            <@{tag} class={classes}>
                {props.children.clone()}
            </@>
        }
    }
}
