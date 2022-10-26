#![allow(clippy::redundant_closure_call)]

use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ControlProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
    /// A modifier to have the controlled element fill up the remaining space.
    #[prop_or_default]
    pub expanded: bool,
}

/// A container with which you can wrap the form controls.
///
/// [https://bulma.io/documentation/form/general/](https://bulma.io/documentation/form/general/)
#[function_component(Control)]
pub fn conttrol(ControlProps { children, classes, tag, expanded }: &ControlProps) -> Html {
    let classes = classes!(classes.clone(), "control", expanded.then_some("is-expanded"));

    html! {
        <@{tag.clone()} class={classes}>
            {children.clone()}
        </@>
    }
}
