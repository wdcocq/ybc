use derive_more::Display;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct HeroProps {
    /// Extra classes for the hero container.
    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The contents of the hero-head section.
    #[prop_or_default]
    pub head: Option<Html>,
    /// Optional classes to add to the hero-head container.
    #[prop_or_default]
    pub head_classes: Option<Classes>,
    /// The contents of the hero-body section.
    pub body: Html,
    /// Optional classes to add to the hero-body container.
    #[prop_or_default]
    pub body_classes: Option<Classes>,
    /// The contents of the hero-foot section.
    #[prop_or_default]
    pub foot: Option<Html>,
    /// Optional classes to add to the hero-foot container.
    #[prop_or_default]
    pub foot_classes: Option<Classes>,
    /// If you are using a [fixed navbar](https://bulma.io/documentation/components/navbar/#fixed-navbar),
    /// you can use the `fixed_nav=true` modifier on the hero for it to occupy the viewport height minus
    /// the navbar height.
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
pub struct Hero; 

impl Component for Hero {
    type Message = ();
    type Properties = HeroProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("hero");
        classes.push(&props.classes);
        if props.fixed_nav {
            classes.push("is-fullheight-with-navbar");
        }
        if props.bold {
            classes.push("is-bold");
        }
        if let Some(size) = &props.size {
            classes.push(&size.to_string());
        }

        // Build the header section.
        let head = if let Some(head) = &props.head {
            let mut classes = Classes::from("hero-head");
            classes.push(&props.head_classes);
            html! {<div class={classes}>{head.clone()}</div>}
        } else {
            html! {}
        };
        // Build the footer section.
        let foot = if let Some(foot) = &props.foot {
            let mut classes = Classes::from("hero-foot");
            classes.push(&props.foot_classes);
            html! {<div class={classes}>{foot.clone()}</div>}
        } else {
            html! {}
        };

        let mut body_classes = Classes::from("hero-body");
        body_classes.push(&props.body_classes);
        html! {
            <section class={classes}>
                {head}
                <div class={body_classes}>{props.body.clone()}</div>
                {foot}
            </section>
        }
    }
}

/// The 4 sizes available for heros.
///
/// [https://bulma.io/documentation/layout/hero/#sizes](https://bulma.io/documentation/layout/hero/#sizes)
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum HeroSize {
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
    #[display(fmt = "fullheight")]
    Fullheight,
    #[display(fmt = "fullheight-with-navbar")]
    FullheightWithNavbar,
}
