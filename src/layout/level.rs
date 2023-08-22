#![allow(clippy::redundant_closure_call)]

use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "nav".into())]
    pub tag: AttrValue,
}

/// A multi-purpose horizontal level, which can contain almost any other element.
///
/// <https://bulma.io/documentation/layout/level/>
#[function_component(Level)]
pub fn level(LevelProps { children, classes, tag }: &LevelProps) -> Html {
    basic_comp!(<@tag>, children, classes.clone(), "level")
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LevelChildProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: AttrValue,
}

/// A container for level elements to be grouped to the left of the container.
///
/// <https://bulma.io/documentation/layout/level/>
#[function_component(LevelLeft)]
pub fn level_left(LevelChildProps { children, classes, tag }: &LevelChildProps) -> Html {
    basic_comp!(<@tag>, children, classes.clone(), "level-left")
}

/// A container for level elements to be grouped to the right of the container.
///
/// <https://bulma.io/documentation/layout/level/>
#[function_component(LevelRight)]
pub fn level_right(LevelChildProps { children, classes, tag }: &LevelChildProps) -> Html {
    basic_comp!(<@tag>, children, classes.clone(), "level-right")
}

/// An individual element of a level container.
///
/// <https://bulma.io/documentation/layout/level/>
#[function_component(LevelItem)]
pub fn level_item(LevelChildProps { children, classes, tag }: &LevelChildProps) -> Html {
    basic_comp!(<@tag>, children, classes.clone(), "level-item")
}
