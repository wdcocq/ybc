use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BlockProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// Bulma’s most basic spacer block
///
/// [https://bulma.io/documentation/elements/block/](https://bulma.io/documentation/elements/block/)
pub struct Block;

impl Component for Block {
    type Message = ();
    type Properties = BlockProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("block");
        classes.push(&props.classes);
        html! {
            <div class={classes}>
                {props.children.clone()}
            </div>
        }
    }
}
