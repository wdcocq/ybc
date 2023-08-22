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
    pub tag: AttrValue,
    /// A modifier to have the controlled element fill up the remaining space.
    #[prop_or_default]
    pub expanded: bool,
    #[prop_or_default]
    pub has_icons_left: bool,
    #[prop_or_default]
    pub has_icons_right: bool,
}

/// A container with which you can wrap the form controls.
///
/// <https://bulma.io/documentation/form/general/>
#[function_component(Control)]
pub fn conttrol(
    ControlProps {
        children,
        classes,
        tag,
        expanded,
        has_icons_left,
        has_icons_right,
    }: &ControlProps,
) -> Html {
    basic_comp!(<@tag>, children,
                classes.clone(), "control",
                expanded.then_some("is-expanded"),
                has_icons_left.then_some("has-icons-left"),
                has_icons_right.then_some("has-icons-right"))
}
