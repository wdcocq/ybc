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
    pub tag: AttrValue,
    /// The click handler for this component.
    #[prop_or_default]
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
/// <https://bulma.io/documentation/elements/tag/>
#[function_component(Tag)]
pub fn tag(
    TagProps {
        children,
        classes,
        tag,
        onclick,
        rounded,
        delete,
        size,
    }: &TagProps,
) -> Html {
    basic_comp!(
        <@tag [{onclick}]>,
        children,
        classes.clone(),
        "tag",
        rounded.then_some("is-rounded"),
        delete.then_some("is-delete"),
        size
    )
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
/// <https://bulma.io/documentation/elements/tag/>
#[function_component(Tags)]
pub fn tags(TagsProps { children, classes, has_addons }: &TagsProps) -> Html {
    basic_comp!(<div>, children, classes.clone(), "tags", has_addons.then_some("has-addons"))
}
