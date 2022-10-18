#![allow(clippy::redundant_closure_call)]

use yew::events::MouseEvent;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The HTML content of this panel's heading; it is automatically wrapped in a
    /// `p.panel-heading`.
    #[prop_or_default]
    pub heading: Html,
}

/// A composable panel, for compact controls.
///
/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
#[function_component(Panel)]
pub fn panel(PanelProps { children, classes, heading }: &PanelProps) -> Html {
    let classes = classes!(classes.clone(), "panel");

    html! {
        <nav class={classes}>
            <p class="panel-heading">{heading.clone()}</p>
            {children.clone()}
        </nav>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelTabsProps {
    #[prop_or_default]
    pub children: Children,
}

/// A container for the navigation tabs of a panel.
///
/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
#[function_component(PanelTabs)]
pub fn panel_tabs(PanelTabsProps { children }: &PanelTabsProps) -> Html {
    html! {
        <p class="panel-tabs">
            {children.clone()}
        </p>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelBlockProps {
    #[prop_or_default]
    pub children: Children,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
    /// Make this element the active / highlighted element.
    #[prop_or_default]
    pub active: bool,
    /// The click handler for this element.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

/// An individual element of the panel.
///
/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
#[function_component(PanelBlock)]
pub fn panel_block(PanelBlockProps { children, tag, active, onclick }: &PanelBlockProps) -> Html {
    let classes = classes!("panel-block", active.then_some("is-active"));

    html! {
        <@{tag.clone()} class={classes} {onclick}>
            {children.clone()}
        </@>
    }
}
