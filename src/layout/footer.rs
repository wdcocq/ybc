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
#[function_component(Footer)]
pub fn footer(FooterProps { children, classes }: &FooterProps) -> Html {
    basic_comp!(<footer>, children, classes.clone(), "footer")
}
