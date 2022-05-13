use derive_more::Display;
use yew::events::MouseEvent;
use yew::prelude::*;

use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct PaginationProps {
    /// The child `li`, `pagination-link` & `pagination-ellipsis` elements for pagination.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
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
/// [https://bulma.io/documentation/components/pagination/](https://bulma.io/documentation/components/pagination/)
pub struct Pagination;

impl Component for Pagination {
    type Message = ();
    type Properties = PaginationProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("pagination");
        classes.push(&props.classes);
        if let Some(size) = &props.size {
            classes.push(&size.to_string());
        }
        if let Some(alignment) = &props.alignment {
            classes.push(&alignment.to_string());
        }
        if props.rounded {
            classes.push("is-rounded");
        }
        html! {
            <nav class={classes} role="navigation" aria-label="pagination">
                {props.previous.clone()}
                {props.next.clone()}
                <ul class="pagination-list">
                    {props.children.clone()}
                </ul>
            </nav>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
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
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

/// A pagination element representing a link to a page number, the previous page or the next page.
///
/// [https://bulma.io/documentation/components/pagination/](https://bulma.io/documentation/components/pagination/)
pub struct PaginationItem;

impl Component for PaginationItem {
    type Message = ();
    type Properties = PaginationItemProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
            <a class={props.item_type.to_string()} aria-label={props.label.clone()} onclick={props.onclick.clone()}>
                {props.children.clone()}
            </a>
        }
    }
}

/// A pagination item type.
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "pagination-{}")]
pub enum PaginationItemType {
    /// A pagination link for a specific page number.
    #[display(fmt = "link")]
    Link,
    /// A pagination button for the next page.
    #[display(fmt = "next")]
    Next,
    /// A pagination button for the previous page.
    #[display(fmt = "previous")]
    Previous,
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

/// A horizontal ellipsis for pagination range separators.
///
/// [https://bulma.io/documentation/components/pagination/](https://bulma.io/documentation/components/pagination/)
pub struct PaginationEllipsis;

impl Component for PaginationEllipsis {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {<span class="pagination-ellipsis">{"&hellip;"}</span>}
    }
}

//////////////////////////////////////////////////////////////////////////////
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
    pub struct PaginationItemRouter<R>
    where
        R: Routable + 'static,
    {
        marker: std::marker::PhantomData<R>,
    }

    impl<R> Component for PaginationItemRouter<R>
    where
        R: Routable + 'static,
    {
        type Message = ();
        type Properties = RouterProps<R>;

        fn create(_ctx: &Context<Self>) -> Self {
            Self { marker: std::marker::PhantomData }
        }

        #[allow(deprecated)]
        fn view(&self, ctx: &Context<Self>) -> Html {
            let props = ctx.props();

            html! {
                <Link<R>
                    to={props.route.clone()}
                    children={props.children.clone()}
                    classes={classes!(props.item_type.to_string())}
                />
            }
        }
    }
}

#[cfg(feature = "router")]
pub use router::PaginationItemRouter;
