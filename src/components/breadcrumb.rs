use strum::IntoStaticStr;
use yew::prelude::*;

use crate::Alignment;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BreadcrumbProps {
    /// The `li` child elements of this breadcrumb.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
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
    let classes = classes!(classes.clone(), "breadcrumb", size, alignment, separator);

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
#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum BreadcrumbSize {
    #[strum(to_string = "is-small")]
    Small,
    #[strum(to_string = "is-medium")]
    Medium,
    #[strum(to_string = "is-large")]
    Large,
}

/// The 4 additional separators for a breadcrump.
///
/// https://bulma.io/documentation/components/breadcrumb/#alternative-separators
#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum BreadcrumbSeparator {
    #[strum(to_string = "has-arrow-seperator")]
    Arrow,
    #[strum(to_string = "has-bullet-seperator")]
    Bullet,
    #[strum(to_string = "has-dot-seperator")]
    Dot,
    #[strum(to_string = "has-succeeds-seperator")]
    Succeeds,
}

impl_classes_from!(BreadcrumbSize, BreadcrumbSeparator);
