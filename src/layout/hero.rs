use strum::IntoStaticStr;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct HeroProps {
    /// Extra classes for the hero container.
    #[prop_or_default]
    pub classes: Classes,
    /// The contents of the hero-head section.
    #[prop_or_default]
    pub head: Option<Html>,
    /// Optional classes to add to the hero-head container.
    #[prop_or_default]
    pub head_classes: Classes,
    /// The contents of the hero-body section.
    pub body: Html,
    /// Optional classes to add to the hero-body container.
    #[prop_or_default]
    pub body_classes: Classes,
    /// The contents of the hero-foot section.
    #[prop_or_default]
    pub foot: Option<Html>,
    /// Optional classes to add to the hero-foot container.
    #[prop_or_default]
    pub foot_classes: Classes,
    /// If you are using a [fixed navbar](https://bulma.io/documentation/components/navbar/#fixed-navbar),
    /// you can use the `fixed_nav=true` modifier on the hero for it to occupy the viewport height
    /// minus the navbar height.
    ///
    /// https://bulma.io/documentation/layout/hero/#fullheight-with-navbar
    #[prop_or_default]
    pub fixed_nav: bool,
    /// Generate a subtle gradient for the hero.
    #[prop_or_default]
    pub bold: bool,
    /// The size for this hero.
    #[prop_or_default]
    pub size: Option<HeroSize>,
}

/// An imposing hero banner to showcase something.
///
/// [https://bulma.io/documentation/layout/hero/](https://bulma.io/documentation/layout/hero/)
#[function_component(Hero)]
pub fn hero(
    HeroProps {
        classes,
        head,
        head_classes,
        body,
        body_classes,
        foot,
        foot_classes,
        fixed_nav,
        bold,
        size,
    }: &HeroProps,
) -> Html {
    let classes = classes!(
        classes.clone(),
        "hero",
        fixed_nav.then_some("is-fullheight-with-navbar"),
        bold.then_some("is-bold"),
        size,
    );

    html! {
        <section class={classes}>
            if let Some(head) = head {
                <div class={classes!(head_classes.clone(), "hero-head")}>
                    {head.clone()}
                </div>
            }
            <div class={classes!(body_classes.clone(), "hero-body")}>
                {body.clone()}
            </div>
            if let Some(foot) = foot {
                <div class={classes!(foot_classes.clone(), "hero-foot")}>
                    {foot.clone()}
                </div>
            }
        </section>
    }
}

/// The 4 sizes available for heros.
///
/// [https://bulma.io/documentation/layout/hero/#sizes](https://bulma.io/documentation/layout/hero/#sizes)
#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum HeroSize {
    #[strum(to_string = "is-medium")]
    Medium,
    #[strum(to_string = "is-large")]
    Large,
    #[strum(to_string = "is-fullheight")]
    Fullheight,
    #[strum(to_string = "is-fullheight-with-navbar")]
    FullheightWithNavbar,
}

impl_classes_from!(HeroSize);
