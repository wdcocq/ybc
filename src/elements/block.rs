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
/// <https://bulma.io/documentation/elements/block/>
#[function_component(Block)]
pub fn block(BlockProps { children, classes }: &BlockProps) -> Html {
    basic_comp!(<div>, children, classes.clone(), "block")
}
