use derive_more::Display;
use yew::prelude::*;

use crate::Alignment;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BreadcrumbProps {
    /// The `li` child elements of this breadcrumb.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<BreadcrumbSize>,
    /// The alignment of this component.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    /// The separator type to use between breadcrumb segments.
    #[prop_or_default]
    pub separator: Option<BreadcrumbSeparator>,
}

/// A simple breadcrumb component to improve your navigation experience.
///
/// [https://bulma.io/documentation/components/breadcrumb/](https://bulma.io/documentation/components/breadcrumb/)
#[function_component(Breadcrumb)]
pub fn breadcrumb(BreadcrumbProps { children, classes, size, alignment, separator }: &BreadcrumbProps) -> Html {
    let classes = classes!(classes, "breadcrumb", size, alignment, separator);

    html! {
        <nav class={classes} aria-label="breadcrumbs">
            <ul>
                {children.clone()}
            </ul>
        </nav>
    }
}

/// The 3 sizes available for a breadcrumb.
///
/// https://bulma.io/documentation/components/breadcrumb/#sizes
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum BreadcrumbSize {
    #[display(fmt = "small")]
    Small,
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
}

impl From<BreadcrumbSize> for Classes {
    fn from(size: BreadcrumbSize) -> Self {
        size.to_string().into()
    }
}

/// The 4 additional separators for a breadcrump.
///
/// https://bulma.io/documentation/components/breadcrumb/#alternative-separators
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "has-{}-separator")]
pub enum BreadcrumbSeparator {
    #[display(fmt = "arrow")]
    Arrow,
    #[display(fmt = "bullet")]
    Bullet,
    #[display(fmt = "dot")]
    Dot,
    #[display(fmt = "succeeds")]
    Succeeds,
}

impl From<BreadcrumbSeparator> for Classes {
    fn from(separator: BreadcrumbSeparator) -> Self {
        separator.to_string().into()
    }
}
