#![allow(clippy::redundant_closure_call)]

use derive_more::Display;
use yew::events::InputEvent;
use yew::prelude::*;

use crate::Size;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct InputProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub value: Option<String>,
    /// The callback to be used for propagating changes to this element's value.
    #[prop_or_default]
    pub update: Callback<InputEvent>,

    #[prop_or_default]
    pub classes: Option<Classes>,
    /// The input type of this component.
    #[prop_or(InputType::Text)]
    pub r#type: InputType,
    /// The placeholder value for this component.
    #[prop_or_default]
    pub placeholder: String,
    /// Datalist id
    #[prop_or_default]
    pub list: Option<String>,
    #[prop_or(true)]
    pub autocomplete: bool,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Use rounded appearance.
    #[prop_or_default]
    pub rounded: bool,
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

/// A text input element.
///
/// [https://bulma.io/documentation/form/input/](https://bulma.io/documentation/form/input/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
pub struct Input;

impl Component for Input {
    type Message = InputEvent;
    type Properties = InputProps;

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
        let classes = classes!(
            "input",
            &props.classes,
            props.size,
            props.rounded.then_some("is-rounded"),
            props.loading.then_some("is-loading"),
            props.r#static.then_some("is-static"),
        );

        let autocomplete = if props.autocomplete { "on" } else { "off" };

        html! {
            <input
                name={props.name.clone()}
                value={props.value.clone()}
                oninput={link.callback(std::convert::identity)}
                class={classes}
                type={props.r#type.to_string()}
                placeholder={props.placeholder.clone()}
                list={props.list.clone()}
                autocomplete={autocomplete}
                disabled={props.disabled}
                readonly={props.readonly}
                />
        }
    }
}

/// The 4 allowed types for an input component.
///
/// https://bulma.io/documentation/form/input/
#[derive(Clone, Debug, Display, PartialEq)]
pub enum InputType {
    #[display(fmt = "text")]
    Text,
    #[display(fmt = "password")]
    Password,
    #[display(fmt = "email")]
    Email,
    #[display(fmt = "tel")]
    Tel,
}
