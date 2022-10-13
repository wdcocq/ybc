use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BoxProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A white box to contain other elements.
///
/// [https://bulma.io/documentation/elements/box/](https://bulma.io/documentation/elements/box/)
pub struct Box;

impl Component for Box {
    type Message = ();
    type Properties = BoxProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("box");
        classes.push(&props.classes);
        html! {
            <div class={classes}>
                {props.children.clone()}
            </div>
        }
    }
}
