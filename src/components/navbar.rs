#![allow(clippy::redundant_closure_call)]

use std::ops::Deref;

use derive_more::{Deref, Display};
use yew::prelude::*;

use crate::components::dropdown::DropdownMsg;

/// Reducer to keep menu active state.
#[derive(Copy, Clone, Debug, Deref, PartialEq)]
struct NavbarActive(bool);

enum NavbarActiveAction {
    Toggle,
    Open,
    Close,
}

impl Reducible for NavbarActive {
    type Action = NavbarActiveAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            NavbarActiveAction::Toggle => Self(!self.0).into(),
            NavbarActiveAction::Open => Self(true).into(),
            NavbarActiveAction::Close => Self(false).into(),
        }
    }
}

impl From<NavbarActive> for Classes {
    fn from(active: NavbarActive) -> Self {
        classes!(active.then(|| "is-active"))
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NavbarProps {
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// Make the navbar fixed to the top or bottom of the UI.
    #[prop_or_default]
    pub fixed: Option<NavbarFixed>,
    /// Seamlessly integrate the navbar in any visual context.
    ///
    /// [https://bulma.io/documentation/components/navbar/#transparent-navbar](https://bulma.io/documentation/components/navbar/#transparent-navbar)
    #[prop_or_default]
    pub transparent: bool,
    /// Sets **top** and **bottom** paddings with **1rem**, **left** and **right** paddings with
    /// **2rem**.
    ///
    /// [https://bulma.io/documentation/components/navbar/#navbar-helper-classes](https://bulma.io/documentation/components/navbar/#navbar-helper-classes)
    #[prop_or_default]
    pub spaced: bool,
    /// The contents of the navbar brand. The `navbar-burger` is automatically appended to the
    /// end of this content.
    ///
    /// [https://bulma.io/documentation/components/navbar/#navbar-brand](https://bulma.io/documentation/components/navbar/#navbar-brand)
    /// If true, the contents of the navbar will be wrapped in a container.
    #[prop_or_default]
    pub padded: bool,
    /// The contents of the `navbar-brand` section of the navbar.
    #[prop_or_default]
    pub navbrand: Option<Html>,
    /// The contents of the `navbar-start` section of the navbar.
    #[prop_or_default]
    pub navstart: Option<Html>,
    /// The contents of the `navbar-end` section of the navbar.
    #[prop_or_default]
    pub navend: Option<Html>,
    /// A bool controlling if the navbar should have a navbar burger for smaller viewports.
    #[prop_or_else(|| true)]
    pub navburger: bool,
    /// Extra classes for the navbar burger.
    #[prop_or_default]
    pub navburger_classes: Option<Classes>,
    /// Aria label of the navbar
    #[prop_or_else(|| "main navigation".to_string())]
    pub aria_label: String,
}

/// A responsive horizontal navbar that can support images, links, buttons, and dropdowns.
///
/// [https://bulma.io/documentation/components/navbar/](https://bulma.io/documentation/components/navbar/)
#[function_component(Navbar)]
pub fn navbar(
    NavbarProps {
        classes,
        fixed,
        transparent,
        spaced,
        padded,
        navbrand,
        navstart,
        navend,
        navburger,
        navburger_classes,
        aria_label,
    }: &NavbarProps,
) -> Html {
    let is_menu_active = use_reducer_eq(|| NavbarActive(false));

    let classes = classes!(classes, "navbar", fixed);
    let nav_classes = classes!("navbar-menu", *is_menu_active);
    let burger_classes = classes!(navburger_classes, "navbar-burger", *is_menu_active);

    let onclick = use_callback(
        |_, dispatcher| dispatcher.dispatch(NavbarActiveAction::Toggle),
        is_menu_active.dispatcher(),
    );

    let contents = html! {
        <>
            if let Some(navbrand) = navbrand {
                <div class="navbar-brand">
                    {navbrand.clone()}
                    if *navburger {
                        <a class={burger_classes} {onclick}
                            role="button" aria-label="menu"
                            aria-expanded={format!("{}", **is_menu_active)}
                        >
                            <span aria-hidden="true"></span>
                            <span aria-hidden="true"></span>
                            <span aria-hidden="true"></span>
                        </a>
                    }
                </div>
            }
            <div class={nav_classes}>
                if let Some(navstart) = navstart {
                    <div class="navbar-start">{navstart.clone()}</div>
                }
                if let Some(navend) = navend {
                    <div class="navbar-end">{navend.clone()}</div>
                }
            </div>
        </>
    };

    html! {
        <nav class={classes} role="navigation" aria-label={aria_label.clone()}>
            if *padded {
                <div class="container">{contents}</div>
            } else {
                {contents}
            }
        </nav>
    }
}

/// The 2 possible fixed positions available for a navbar.
///
/// [https://bulma.io/documentation/components/navbar/#fixed-navbar](https://bulma.io/documentation/components/navbar/#fixed-navbar)
///
/// NOTE WELL: in order to work properly, the root `html` or `body` element must be configured with
/// the corresponding `has-navbar-fixed-top` or `has-navbar-fixed-bottom` class.
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum NavbarFixed {
    #[display(fmt = "fixed-top")]
    Top,
    #[display(fmt = "fixed-bottom")]
    Bottom,
}

impl From<NavbarFixed> for Classes {
    fn from(fixed: NavbarFixed) -> Self {
        fixed.to_string().into()
    }
}

//////////////////////////////////////////////////////////////////////////////

/// The two HTML tags allowed for a navbar-item.
///
/// [https://bulma.io/documentation/components/navbar/#navbar-item](https://bulma.io/documentation/components/navbar/#navbar-item)
#[derive(Clone, Debug, Display, PartialEq)]
pub enum NavbarItemTag {
    #[display(fmt = "a")]
    A,
    #[display(fmt = "div")]
    Div,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NavbarItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The HTML tag to use for this component.
    #[prop_or(NavbarItemTag::Div)]
    pub tag: NavbarItemTag,
    /// Add the `has-dropdown` class to this element, indicating that it is the parent
    /// of a dropdown menu.
    #[prop_or_default]
    pub has_dropdown: bool,
    /// Turn this into a full-width element.
    #[prop_or_default]
    pub expanded: bool,
    /// Add a bottom border on hover, and show the bottom border using `is_active=true`.
    #[prop_or_default]
    pub tab: bool,
    /// Show the bottom border when `is_tab=true`.
    #[prop_or_default]
    pub active: bool,
    /// An optional `href` for when this element is using the `a` tag.
    #[prop_or_default]
    pub href: Option<String>,
    /// An optional `rel` for when this element is using the `a` tag.
    #[prop_or_default]
    pub rel: Option<String>,
    /// An optional `target` for when this element is using the `a` tag.
    #[prop_or_default]
    pub target: Option<String>,
}

/// A single element of the navbar.
///
/// [https://bulma.io/documentation/components/navbar/](https://bulma.io/documentation/components/navbar/)
#[function_component(NavbarItem)]
pub fn navbar_item(
    NavbarItemProps {
        children,
        classes,
        tag,
        has_dropdown,
        expanded,
        tab,
        active,
        href,
        rel,
        target,
    }: &NavbarItemProps,
) -> Html {
    let mut classes = classes!(
        classes,
        "navbar-item",
        has_dropdown.then(|| "has-dropdown"),
        expanded.then(|| "is-expanded"),
        tab.then(|| "is-tab"),
        active.then(|| "is-active"),
    );

    match tag {
        NavbarItemTag::A => html! {
            <a
                class={classes}
                href={href.clone().unwrap_or_default()}
                rel={rel.clone().unwrap_or_default()}
                target={target.clone().unwrap_or_default()}
            >
                {children.clone()}
            </a>
        },
        NavbarItemTag::Div => html! {
            <div class={classes}>
                {children.clone()}
            </div>
        },
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NavbarDividerProps {
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// An element to display a horizontal rule in a navbar-dropdown.
///
/// [https://bulma.io/documentation/components/navbar/#dropdown-menu](https://bulma.io/documentation/components/navbar/#dropdown-menu)
#[function_component(NavbarDivider)]
pub fn navbar_divider(NavbarDividerProps { classes }: &NavbarDividerProps) -> Html {
    let classes = classes!(classes, "navbar-divider");
    html! {
        <hr class={classes}/>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NavbarDropdownProps {
    /// The content of the dropdown; these should all be `NavbarItems` & `NavbarDividers`.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The contents of the navbar-link used for triggering the dropdown menu.
    pub navlink: Html,
    /// Make this dropdown triggerable based on hover.
    #[prop_or_default]
    pub hoverable: bool,
    /// Configure this manu to be a dropup.
    #[prop_or_default]
    pub dropup: bool,
    /// Render the contents of this dropdown to the right.
    #[prop_or_default]
    pub right: bool,
    /// Remove the arrow from the dropdown menu trigger.
    #[prop_or_default]
    pub arrowless: bool,
    /// Use the boxed style for the dropdown, typically coupled with a transparent navbar.
    #[prop_or_default]
    pub boxed: bool,
}

/// A navbar dropdown menu, which can include navbar items and dividers.
///
/// This component is a composite of all of the elements needed in order to properly generate
/// a navbar dropdown component.
///
/// [https://bulma.io/documentation/components/navbar/#dropdown-menu](https://bulma.io/documentation/components/navbar/#dropdown-menu)
#[function_component(NavbarDropdown)]
pub fn navbar_dropdown(
    NavbarDropdownProps {
        children,
        classes,
        navlink,
        hoverable,
        dropup,
        right,
        arrowless,
        boxed,
    }: &NavbarDropdownProps,
) -> Html {
    let is_menu_active = use_reducer_eq(|| NavbarActive(false));
    let opencb = use_callback(|_, dispatcher| dispatcher.dispatch(NavbarActiveAction::Open), is_menu_active.dispatcher());
    let closecb = use_callback(
        |_, dispatcher| dispatcher.dispatch(NavbarActiveAction::Close),
        is_menu_active.dispatcher(),
    );

    let classes = classes!(
        classes,
        "navbar-item",
        "has-dropdown",
        dropup.then(|| "has-dropdown-up"),
        hoverable.then(|| "is-hoverable"),
        *is_menu_active,
    );
    let drop_classes = classes!("navbar-dropdown", right.then(|| "is-right"), boxed.then(|| "is-boxed"));
    let link_classes = classes!("navbar-link", arrowless.then(|| "is-arrowless"));

    html! {
        <div class={classes}>
            if **is_menu_active {
                <div onclick={closecb} style="z-index:10;background-color:rgba(0,0,0,0);position:fixed;top:0;bottom:0;left:0;right:0;"/>
            }
            <a class={link_classes} onclick={opencb}>{navlink.clone()}</a>
            <div class={drop_classes}>
                {children.clone()}
            </div>
        </div>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "router")]
mod router {
    use super::*;
    use yew_router::components::Link;
    use yew_router::hooks::use_route;
    use yew_router::Routable;

    #[derive(Clone, Properties, PartialEq)]
    pub struct RouterProps<R>
    where
        R: Routable,
    {
        /// The Switched item representing the route.
        pub to: R,
        /// Html inside the component.
        #[prop_or_default]
        pub children: Children,
        #[prop_or_default]
        pub classes: Option<Classes>,
        /// Turn this into a full-width element.
        #[prop_or_default]
        pub expanded: bool,
        /// Add a bottom border on hover, and show the bottom border using `is_active=true`.
        #[prop_or_default]
        pub tab: bool,
    }

    #[function_component(NavbarItemRouter)]
    pub fn navbar_item_router<R>(RouterProps::<R> { to, children, classes, expanded, tab }: &RouterProps<R>) -> Html
    where
        R: Routable + 'static,
    {
        let route = use_route::<R>();
        let classes = classes!(
            classes,
            "navbar-item",
            expanded.then(|| "is-expanded"),
            tab.then(|| "is-tab"),
            route.filter(|route| route == to).map(|_| "is-active")
        );

        html! {
            <Link<R> {classes} to={to.clone()}>
                {children.clone()}
            </Link<R>>
        }
    }
}

#[cfg(feature = "router")]
pub use router::NavbarItemRouter;
