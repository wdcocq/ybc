use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BlockProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

/// Bulma’s most basic spacer block
///
/// [https://bulma.io/documentation/elements/block/](https://bulma.io/documentation/elements/block/)
#[function_component(Block)]
pub fn block(BlockProps { children, classes }: &BlockProps) -> Html {
    let classes = classes!(classes.clone(), "block");

    html! {
        <div class={classes}>
            {children.clone()}
        </div>
    }
}
