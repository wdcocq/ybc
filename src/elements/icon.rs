use yew::events::MouseEvent;
use yew::prelude::*;

use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct IconProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The click handler to use for this component.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// The size of this component; to help prevent page "jumps" during load.
    #[prop_or_default]
    pub size: Option<Size>,
    /// The alignment of this icon, often used within form controls.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
}

/// A container for any type of icon font.
///
/// <https://bulma.io/documentation/elements/icon/>
#[function_component(Icon)]
pub fn icon(IconProps { children, classes, onclick, size, alignment }: &IconProps) -> Html {
    basic_comp!(<span [{onclick}]>, children, classes.clone(), "icon", size, alignment)
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct IconTextProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The click handler to use for this component.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

/// A wrapper to combine text with icons. Text should be wrapped in a `<span>` tag
///
/// <https://bulma.io/documentation/elements/icon/>
#[function_component(IconText)]
pub fn icon_text(IconTextProps { children, classes, onclick }: &IconTextProps) -> Html {
    basic_comp!(<span [{onclick}]>, children, classes.clone(), "icon-text")
}
