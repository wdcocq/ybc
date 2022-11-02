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
    let classes = classes!(classes.clone(), "card");

    html! {
        <div ref={card_ref} class={classes}>
            {children.clone()}
        </div>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardHeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

/// A container for card header content; rendered as a horizontal bar with a shadow.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
#[function_component(CardHeader)]
pub fn card_header(CardHeaderProps { children, classes }: &CardHeaderProps) -> Html {
    let classes = classes!(classes.clone(), "card-header");

    html! {
        <header class={classes}>
            {children.clone()}
        </header>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardImageProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

/// A fullwidth container for a responsive image.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
#[function_component(CardImage)]
pub fn card_image(CardImageProps { children, classes }: &CardImageProps) -> Html {
    let classes = classes!(classes.clone(), "card-image");

    html! {
        <div class={classes}>
            {children.clone()}
        </div>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

/// A container for any other content as the body of the card.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
#[function_component(CardContent)]
pub fn card_content(CardContentProps { children, classes }: &CardContentProps) -> Html {
    let classes = classes!(classes.clone(), "card-content");

    html! {
        <div class={classes}>
            {children.clone()}
        </div>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CardFooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

/// A container for card footer content; rendered as a horizontal list of controls.
///
/// [https://bulma.io/documentation/components/card/](https://bulma.io/documentation/components/card/)
#[function_component(CardFooter)]
pub fn card_footer(CardFooterProps { children, classes }: &CardFooterProps) -> Html {
    let classes = classes!(classes.clone(), "card-footer");

    html! {
        <footer class={classes}>
            {children.clone()}
        </footer>
    }
}
