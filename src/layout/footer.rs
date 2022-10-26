use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

/// A simple responsive footer which can include anything.
///
/// [https://bulma.io/documentation/layout/footer/](https://bulma.io/documentation/layout/footer/)
pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = FooterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("footer");
        classes.push(props.classes.clone());
        html! {
            <footer class={classes}>
                {props.children.clone()}
            </footer>
        }
    }
}
