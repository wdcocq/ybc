use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NotificationProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// Bold notification blocks, to alert your users of something.
///
/// [https://bulma.io/documentation/elements/notification/](https://bulma.io/documentation/elements/notification/)
pub struct Notification;

impl Component for Notification {
    type Message = ();
    type Properties = NotificationProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("notification");
        classes.push(&props.classes);
        html! {
            <div class={classes}>
                {props.children.clone()}
            </div>
        }
    }
}
