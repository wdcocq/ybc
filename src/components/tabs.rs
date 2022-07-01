use yew::prelude::*;

use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TabsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The alignment of this component.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Add a more classic style with borders to this component.
    #[prop_or_default]
    pub boxed: bool,
    /// Add the "radio button" style to the elements of this component.
    #[prop_or_default]
    pub toggle: bool,
    /// Make the tab elements of this component rounded.
    #[prop_or_default]
    pub rounded: bool,
    /// Make this component fullwidth.
    #[prop_or_default]
    pub fullwidth: bool,
}

/// Simple responsive horizontal navigation tabs, with different styles.
///
/// [https://bulma.io/documentation/components/tabs/](https://bulma.io/documentation/components/tabs/)
///
/// For integration with Yew Router, it is recommended that the `RouterButton` or `RouterAnchor`
/// components be used as the individual tab elements for this component.
#[function_component(Tabs)]
pub fn tabs(
    TabsProps {
        children,
        classes,
        alignment,
        size,
        boxed,
        toggle,
        rounded,
        fullwidth,
    }: &TabsProps,
) -> Html {
    let classes = classes!(
        classes,
        "tabs",
        alignment,
        size,
        boxed.then_some("is-boxed"),
        toggle.then_some("is-toggle"),
        rounded.then_some("is-rounded"),
        fullwidth.then_some("is-fullwidth")
    );

    html! {
        <div class={classes}>
            <ul>
                {children.clone()}
            </ul>
        </div>
    }
}
