use yew::events::InputEvent;
use yew::prelude::*;

use crate::Size;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TextAreaProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub value: String,
    /// The callback to be used for propagating changes to this element's value.
    #[prop_or_default]
    pub update: Callback<String>,

    #[prop_or_default]
    pub classes: Classes,
    /// The placeholder value for this component.
    #[prop_or_default]
    pub placeholder: String,
    /// The number of rows to which this component will be locked.
    #[prop_or_default]
    pub rows: u32,

    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Fix the size of this component.
    #[prop_or_default]
    pub fixed_size: bool,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// Make this component read-only.
    #[prop_or_default]
    pub readonly: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
}

/// A multiline textarea component.
///
/// [https://bulma.io/documentation/form/textarea/](https://bulma.io/documentation/form/textarea/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
pub struct TextArea;

impl Component for TextArea {
    type Message = String;
    type Properties = TextAreaProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        ctx.props().update.emit(msg);
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();
        let mut classes = Classes::from("textarea");
        classes.push(props.classes.clone());
        if let Some(size) = &props.size {
            classes.push(&size.to_string());
        }
        if props.loading {
            classes.push("is-loading");
        }
        if props.r#static {
            classes.push("is-static");
        }
        if props.fixed_size {
            classes.push("has-fixed-size");
        }

        html! {
            <textarea
                name={props.name.clone()}
                value={props.value.clone()}
                oninput={link.callback(|e: InputEvent| e.data().unwrap_or_default())}
                class={classes}
                rows={props.rows.to_string()}
                placeholder={props.placeholder.clone()}
                disabled={props.disabled}
                readonly={props.readonly}
                />
        }
    }
}
