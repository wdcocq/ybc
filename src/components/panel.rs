#![allow(clippy::redundant_closure_call)]

use yew::events::MouseEvent;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML content of this panel's heading; it is automatically wrapped in a `p.panel-heading`.
    #[prop_or_default]
    pub heading: Html,
}

/// A composable panel, for compact controls.
///
/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
pub struct Panel; 

impl Component for Panel {
    type Message = ();
    type Properties = PanelProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("panel");
        classes.push(&props.classes);
        html! {
            <nav class={classes}>
                <p class="panel-heading">{props.heading.clone()}</p>
                {props.children.clone()}
            </nav>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PanelTabsProps {
    #[prop_or_default]
    pub children: Children,
}

/// A container for the navigation tabs of a panel.
///
/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
pub struct PanelTabs; 

impl Component for PanelTabs {
    type Message = ();
    type Properties = PanelTabsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
            <p class="panel-tabs">
                {props.children.clone()}
            </p>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
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
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

/// An individual element of the panel.
///
/// [https://bulma.io/documentation/components/panel/](https://bulma.io/documentation/components/panel/)
pub struct PanelBlock; 

impl Component for PanelBlock {
    type Message = ();
    type Properties = PanelBlockProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("panel-block");
        if props.active {
            classes.push("is-active");
        }
        let tag = props.tag.clone();
        html! {
            <@{tag} class={classes} onclick={props.onclick.clone()}>
                {props.children.clone()}
            </@>
        }
    }
}
