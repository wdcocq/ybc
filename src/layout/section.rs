use strum::IntoStaticStr;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SectionProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// A size modifier to control spacing.
    #[prop_or_default]
    pub size: Option<SectionSize>,
}

/// A simple container to divide your page into sections.
///
/// [https://bulma.io/documentation/layout/section/](https://bulma.io/documentation/layout/section/)
#[function_component(Section)]
pub fn section(SectionProps { children, classes, size }: &SectionProps) -> Html {
    basic_comp!(<section>, children, classes.clone(), "section", size)
}

/// The 2 sizes available for sections, which controls spacing.
///
/// [https://bulma.io/documentation/layout/section/](https://bulma.io/documentation/layout/section/)
#[derive(Clone, Debug, IntoStaticStr, PartialEq)]
pub enum SectionSize {
    #[strum(to_string = "is-medium")]
    Medium,
    #[strum(to_string = "is-large")]
    Large,
}

impl_classes_from!(SectionSize);
