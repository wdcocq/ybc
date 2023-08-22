#![allow(clippy::redundant_closure_call)]

use strum::IntoStaticStr;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TitleProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "h3".into())]
    pub tag: AttrValue,
    /// Maintain the normal spacing between titles and subtitles.
    #[prop_or_default]
    pub is_spaced: bool,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<HeaderSize>,
}

/// A simple heading to add depth to your page.
///
/// <https://bulma.io/documentation/elements/title/>
#[function_component(Title)]
pub fn title(TitleProps { children, classes, tag, is_spaced, size }: &TitleProps) -> Html {
    basic_comp!(<@tag>, children, classes.clone(), "title", size, is_spaced.then_some("is-spaced"))
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
    pub tag: AttrValue,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<HeaderSize>,
}

/// A simple heading to add depth to your page.
///
/// <https://bulma.io/documentation/elements/title/>
#[function_component(Subtitle)]
pub fn subtitle(SubtitleProps { children, classes, tag, size }: &SubtitleProps) -> Html {
    basic_comp!(<@tag>, children, classes.clone(), "subtitle", size)
}

/// The six sizes available for titles & subtitles.
///
/// <https://bulma.io/documentation/elements/title/#sizes>
#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum HeaderSize {
    #[strum(to_string = "is-1")]
    Is1,
    #[strum(to_string = "is-2")]
    Is2,
    #[strum(to_string = "is-3")]
    Is3,
    #[strum(to_string = "is-4")]
    Is4,
    #[strum(to_string = "is-5")]
    Is5,
    #[strum(to_string = "is-6")]
    Is6,
}

impl_classes_from!(HeaderSize);
