use derive_more::Display;
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
pub struct Section;

impl Component for Section {
    type Message = ();
    type Properties = SectionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("section");
        classes.push(props.classes.clone());
        if let Some(size) = &props.size {
            classes.push(&size.to_string());
        }
        html! {
            <section class={classes}>
                {props.children.clone()}
            </section>
        }
    }
}

/// The 2 sizes available for sections, which controls spacing.
///
/// [https://bulma.io/documentation/layout/section/](https://bulma.io/documentation/layout/section/)
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum SectionSize {
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
}
