use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// NodeRef to the card `<div>`
    #[prop_or_default]
    pub card_ref: NodeRef,
}

/// An all-around flexible and composable component; this is the card container.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
#[function_component(Card)]
pub fn card(CardProps { children, classes, card_ref }: &CardProps) -> Html {
    basic_comp!(<div [ref={card_ref}]>, children, classes.clone(), "card")
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardHeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub icon: Option<Html>,
}

/// A container for card header content; rendered as a horizontal bar with a shadow.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
#[function_component(CardHeader)]
pub fn card_header(CardHeaderProps { children, classes, icon }: &CardHeaderProps) -> Html {
    let classes = classes!(classes.clone(), "card-header");

    html! {
        <header class={classes}>
            <p class="card-header-title">
                {children.clone()}
            </p>
            if let Some(icon) = icon {
                <span class="card-header-icon">
                    {icon.clone()}
                </span>
            }
        </header>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardChildProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

/// A fullwidth container for a responsive image.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
#[function_component(CardImage)]
pub fn card_image(CardChildProps { children, classes }: &CardChildProps) -> Html {
    basic_comp!(<div>, children, classes.clone(), "card-image")
}

/// A container for any other content as the body of the card.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
#[function_component(CardContent)]
pub fn card_content(CardChildProps { children, classes }: &CardChildProps) -> Html {
    basic_comp!(<div>, children, classes.clone(), "card-content")
}

/// A container for card footer content; rendered as a horizontal list of controls.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
#[function_component(CardFooter)]
pub fn card_footer(CardChildProps { children, classes }: &CardChildProps) -> Html {
    basic_comp!(<footer>, children, classes.clone(), "card-footer")
}
