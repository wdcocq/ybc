use derive_more::Display;
use yew::prelude::*;
use yew::{events::MouseEvent, html::IntoPropValue};

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
    let classes = classes!(classes, "pagination", size, alignment, rounded.then(|| "is-rounded"));

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
/// [https://bulma.io/documentation/components/pagination/](https://bulma.io/documentation/components/pagination/)
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
#[derive(Clone, Copy, Debug, Display, PartialEq)]
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

impl From<PaginationItemType> for Classes {
    fn from(item_type: PaginationItemType) -> Self {
        item_type.to_string().into()
    }
}

//////////////////////////////////////////////////////////////////////////////

/// A horizontal ellipsis for pagination range separators.
///
/// [https://bulma.io/documentation/components/pagination/](https://bulma.io/documentation/components/pagination/)
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
