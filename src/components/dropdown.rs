use std::rc::Rc;

use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ControlledDropdownProps {
    /// The content of the dropdown menu.
    ///
    /// This content will be placed directly within the `div.dropdown-content` container.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Make this dropdown triggerable based on hover.
    #[prop_or_default]
    pub hoverable: bool,
    /// The content of the trigger.
    #[prop_or_default]
    pub trigger_html: Html,
    /// Can be used to active or get the current active state of the dropdown
    pub active: UseStateHandle<bool>,
}

/// An interactive dropdown menu for discoverable content. Can be controlled from outside through
/// the active property.
///
/// [https://bulma.io/documentation/components/dropdown/](https://bulma.io/documentation/components/dropdown/)
#[function_component(ControlledDropdown)]
pub fn activated_dropdown(ControlledDropdownProps { children, classes, hoverable, trigger_html, active }: &ControlledDropdownProps) -> Html {
    let classes = classes!(
        "dropdown",
        classes.clone(),
        hoverable.then_some("is-hoverable"),
        active.then_some("is-active"),
    );

    let opencb = Callback::from({
        let active = active.clone();
        move |_| active.set(true)
    });

    let closecb = Callback::from({
        let active = active.clone();
        move |_| active.set(false)
    });

    html! {
        <div class={classes}>
            if **active {
                <div onclick={closecb}
                     style="z-index:10;background-color:rgba(0,0,0,0);position:fixed;top:0;bottom:0;left:0;right:0;"/>
            }
            <div class="dropdown-trigger" onclick={opencb}>
                {trigger_html.clone()}
            </div>
            <div class="dropdown-menu" role="menu">
                <div class="dropdown-content">
                    {children.clone()}
                </div>
            </div>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DropdownProps {
    /// The content of the dropdown menu.
    ///
    /// This content will be placed directly within the `div.dropdown-content` container.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Make this dropdown triggerable based on hover.
    #[prop_or_default]
    pub hoverable: bool,
    /// The content of the trigger.
    #[prop_or_default]
    pub trigger_html: Html,
}

/// An interactive dropdown menu for discoverable content.
///
/// [https://bulma.io/documentation/components/dropdown/](https://bulma.io/documentation/components/dropdown/)
#[function_component(Dropdown)]
pub fn dropdown(DropdownProps { children, classes, hoverable, trigger_html }: &DropdownProps) -> Html {
    let active = use_state_eq(|| false);

    html! {
        <ControlledDropdown
            classes={classes.clone()}
            hoverable={*hoverable}
            trigger_html={trigger_html.clone()}
            {active}>
            {children.clone()}
        </ControlledDropdown>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DropdownItemTags {
    A(Rc<str>),
    Div,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DropdownItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_else(|| DropdownItemTags::Div)]
    pub tag: DropdownItemTags,
}

#[function_component(DropdownItem)]
pub fn dropdown_item(DropdownItemProps { children, classes, tag }: &DropdownItemProps) -> Html {
    let classes = classes!("dropdown-item", classes.clone());
    match tag {
        DropdownItemTags::A(href) => html! {
            <a class={classes} href={href.clone()}>
                {children.clone()}
            </a>
        },
        DropdownItemTags::Div => html! {
            <div class={classes}>
                {children.clone()}
            </div>
        },
    }
}

#[function_component(DropdownDivider)]
pub fn dropdown_divider() -> Html {
    html! {
        <hr class="dropdown-divider"/>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "router")]
mod router {
    use super::*;
    use serde::Serialize;
    use yew_router::prelude::*;

    #[derive(Debug, Clone, PartialEq, Properties)]
    pub struct DropdownLinkProps<R, Q = ()>
    where
        R: Routable,
        Q: Clone + PartialEq + Serialize,
    {
        #[prop_or_default]
        pub children: Children,
        #[prop_or_default]
        pub classes: Classes,
        /// Route that will be pushed when the anchor is clicked.
        pub to: R,
        /// Route query data
        #[prop_or_default]
        pub query: Option<Q>,
        #[prop_or_default]
        pub disabled: bool,
        /// [`NodeRef`] for the <a> element.
        #[prop_or_default]
        pub anchor_ref: NodeRef,
    }

    #[function_component(DropdownLink)]
    pub fn dropdown_link<R, Q = ()>(DropdownLinkProps { children, classes, to, query, disabled, anchor_ref }: &DropdownLinkProps<R, Q>) -> Html
    where
        R: Routable + 'static,
        Q: Clone + PartialEq + Serialize + 'static,
    {
        let classes = classes!("dropdown-item", classes.clone());

        html! {
            <Link<R, Q> {classes}
                to={to.clone()}
                query={query.clone()}
                disabled={*disabled}
                {anchor_ref}>
                {children.clone()}
            </Link<R, Q>>
        }
    }
}

#[cfg(feature = "router")]
pub use router::DropdownLink;
