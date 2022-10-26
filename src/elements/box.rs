use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BoxProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

/// A white box to contain other elements.
///
/// [https://bulma.io/documentation/elements/box/](https://bulma.io/documentation/elements/box/)
#[function_component(Box)]
pub fn r#box(BoxProps { children, classes }: &BoxProps) -> Html {
    let classes = classes!(classes.clone(), "box");

    html! {
        <div class={classes}>
            {children.clone()}
        </div>
    }
}
