use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MessageProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// Colored message blocks, to emphasize part of your page.
///
/// [https://bulma.io/documentation/components/message/](https://bulma.io/documentation/components/message/)
pub struct Message; 

impl Component for Message {
    type Message = ();
    type Properties = MessageProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("message");
        classes.push(&props.classes);
        html! {
            <article class={classes}>
                {props.children.clone()}
            </article>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MessageHeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// An optional message header that can hold a title and a delete element.
///
/// [https://bulma.io/documentation/components/message/](https://bulma.io/documentation/components/message/)
pub struct MessageHeader; 

impl Component for MessageHeader {
    type Message = ();
    type Properties = MessageHeaderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("message-header");
        classes.push(&props.classes);
        html! {
            <div class={classes}>
                {props.children.clone()}
            </div>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MessageBodyProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
}

/// A container for the body of a message.
///
/// [https://bulma.io/documentation/components/message/](https://bulma.io/documentation/components/message/)
pub struct MessageBody; 

impl Component for MessageBody {
    type Message = ();
    type Properties = MessageBodyProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::from("message-body");
        classes.push(&props.classes);
        html! {
            <div class={classes}>
                {props.children.clone()}
            </div>
        }
    }
}
