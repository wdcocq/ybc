#![allow(clippy::redundant_closure_call)]

use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MediaProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: AttrValue,
}

/// A UI element for repeatable and nestable content.
///
/// [https://bulma.io/documentation/layout/media-object/](https://bulma.io/documentation/layout/media-object/)
#[function_component(Media)]
pub fn media(MediaProps { children, classes, tag }: &MediaProps) -> Html {
    basic_comp!(<@tag>, children, classes.clone(), "media")
}

/// Elements to be grouped to the left of the media container.
///
/// [https://bulma.io/documentation/layout/media-object/](https://bulma.io/documentation/layout/media-object/)
#[function_component(MediaLeft)]
pub fn media_left(MediaProps { children, classes, tag }: &MediaProps) -> Html {
    basic_comp!(<@tag>, children, classes.clone(), "media-left")
}

/// Elements to be grouped to the right of the media container.
///
/// [https://bulma.io/documentation/layout/media-object/](https://bulma.io/documentation/layout/media-object/)
#[function_component(MediaRight)]
pub fn media_right(MediaProps { children, classes, tag }: &MediaProps) -> Html {
    basic_comp!(<@tag>, children, classes.clone(), "media-right")
}

/// Elements to be grouped as the center body of the media container.
///
/// [https://bulma.io/documentation/layout/media-object/](https://bulma.io/documentation/layout/media-object/)
#[function_component(MediaContent)]
pub fn media_content(MediaProps { children, classes, tag }: &MediaProps) -> Html {
    basic_comp!(<@tag>, children, classes.clone(), "media-content")
}
