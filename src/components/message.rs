use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MessageProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

/// Colored message blocks, to emphasize part of your page.
///
/// [https://bulma.io/documentation/components/message/](https://bulma.io/documentation/components/message/)
#[function_component(Message)]
pub fn message(MessageProps { children, classes }: &MessageProps) -> Html {
    let classes = classes!(classes.clone(), "message");

    html! {
        <article class={classes}>
            {children.clone()}
        </article>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MessageHeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

/// An optional message header that can hold a title and a delete element.
///
/// [https://bulma.io/documentation/components/message/](https://bulma.io/documentation/components/message/)
#[function_component(MessageHeader)]
pub fn message_header(MessageHeaderProps { children, classes }: &MessageHeaderProps) -> Html {
    let classes = classes!(classes.clone(), "message-header");

    html! {
        <div class={classes}>
            {children.clone()}
        </div>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MessageBodyProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

/// A container for the body of a message.
///
/// [https://bulma.io/documentation/components/message/](https://bulma.io/documentation/components/message/)
#[function_component(MessageBody)]
pub fn message_body(MessageBodyProps { children, classes }: &MessageBodyProps) -> Html {
    let classes = classes!(classes.clone(), "message-body");

    html! {
        <div class={classes}>
            {children.clone()}
        </div>
    }
}
