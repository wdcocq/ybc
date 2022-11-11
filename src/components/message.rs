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
    basic_comp!(<article>, children, classes.clone(), "message")
}

/// An optional message header that can hold a title and a delete element.
///
/// [https://bulma.io/documentation/components/message/](https://bulma.io/documentation/components/message/)
#[function_component(MessageHeader)]
pub fn message_header(MessageProps { children, classes }: &MessageProps) -> Html {
    basic_comp!(<div>, children, classes.clone(), "message-header")
}

/// A container for the body of a message.
///
/// [https://bulma.io/documentation/components/message/](https://bulma.io/documentation/components/message/)
#[function_component(MessageBody)]
pub fn message_body(MessageProps { children, classes }: &MessageProps) -> Html {
    basic_comp!(<div>, children, classes.clone(), "message-body")
}
