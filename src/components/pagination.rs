use strum::IntoStaticStr;
use yew::events::MouseEvent;
use yew::prelude::*;

use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PaginationProps {
    /// The child `li`, `pagination-link` & `pagination-ellipsis` elements for pagination.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// The alignment of this component.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    /// Make the pagination elements rounded.
    #[prop_or_default]
    pub rounded: bool,

    /// The `pagination-previous` element to use.
    pub previous: Html,
    /// The `pagination-next` element to use.
    pub next: Html,
}

/// A responsive, usable, and flexible pagination component.
///
/// <https://bulma.io/documentation/components/pagination/>
#[function_component(Pagination)]
pub fn pagination(
    PaginationProps {
        children,
        classes,
        size,
        alignment,
        rounded,
        previous,
        next,
    }: &PaginationProps,
) -> Html {
    let classes = classes!(classes.clone(), "pagination", size, alignment, rounded.then_some("is-rounded"));

    html! {
        <nav class={classes} role="navigation" aria-label="pagination">
            {previous.clone()}
            {next.clone()}
            <ul class="pagination-list">
                {children.clone()}
            </ul>
        </nav>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PaginationItemProps {
    pub children: Children,
    /// The pagination item type for this component.
    pub item_type: PaginationItemType,
    /// The aria label to use for this element.
    #[prop_or_default]
    pub label: String,
    /// The click handler for this component.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

/// A pagination element representing a link to a page number, the previous page or the next page.
///
/// <https://bulma.io/documentation/components/pagination/>
#[function_component(PaginationItem)]
pub fn pagination_item(PaginationItemProps { children, item_type, label, onclick }: &PaginationItemProps) -> Html {
    let classes = classes!(*item_type);

    html! {
        <a class={classes} aria-label={label.clone()} {onclick}>
            {children.clone()}
        </a>
    }
}

/// A pagination item type.
#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum PaginationItemType {
    /// A pagination link for a specific page number.
    #[strum(to_string = "pagination-link")]
    Link,
    /// A pagination button for the next page.
    #[strum(to_string = "pagination-next")]
    Next,
    /// A pagination button for the previous page.
    #[strum(to_string = "pagination-previous")]
    Previous,
}

impl_classes_from!(PaginationItemType);
//////////////////////////////////////////////////////////////////////////////

/// A horizontal ellipsis for pagination range separators.
///
/// <https://bulma.io/documentation/components/pagination/>
#[function_component(PaginationEllipsis)]
pub fn pagination_ellipsis() -> Html {
    html! {<span class="pagination-ellipsis">{"&hellip;"}</span>}
}

//////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "router")]
mod router {
    use super::*;
    use yew_router::components::Link;
    use yew_router::Routable;

    #[derive(Clone, Properties, PartialEq)]
    pub struct RouterProps<R>
    where
        R: Routable,
    {
        /// The Switched item representing the route.
        pub route: R,
        /// Html inside the component.
        #[prop_or_default]
        pub children: Children,
        /// The pagination item type for this component.
        pub item_type: PaginationItemType,
    }

    /// A Yew Router link for use in a `Pagination` component.

    #[function_component(PaginationItemRouter)]
    pub fn pagination_item_router<R: Routable + 'static>(RouterProps { route, children, item_type }: &RouterProps<R>) -> Html {
        html! {
            <Link<R>
                to={route.clone()}
                children={children.clone()}
                classes={classes!(item_type)}
            />
        }
    }
}

#[cfg(feature = "router")]
pub use router::PaginationItemRouter;
