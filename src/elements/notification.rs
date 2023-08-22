use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NotificationProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

/// Bold notification blocks, to alert your users of something.
///
/// <https://bulma.io/documentation/elements/notification/>
#[function_component(Notification)]
pub fn notification(NotificationProps { children, classes }: &NotificationProps) -> Html {
    basic_comp!(<div>, children, classes.clone(), "notification")
}
