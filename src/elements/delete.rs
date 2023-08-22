#![allow(clippy::redundant_closure_call)]
use yew::events::MouseEvent;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DeleteProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "button".into())]
    pub tag: AttrValue,
    /// The click handler to use for this component.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

/// A versatile delete cross.
///
/// <https://bulma.io/documentation/elements/delete/>
#[function_component(Delete)]
pub fn delete(DeleteProps { children, classes, tag, onclick }: &DeleteProps) -> Html {
    basic_comp!(<@tag [{onclick}]>, children, classes.clone(), "delete")
}
