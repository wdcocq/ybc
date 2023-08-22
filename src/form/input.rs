#![allow(clippy::redundant_closure_call)]

use strum::IntoStaticStr;
use yew::events::InputEvent;
use yew::prelude::*;

use crate::common::Color;
use crate::{BackgroundColor, Size};

properties_with_globals!(
    #[derive(Clone, Debug, Properties, PartialEq)]
    pub struct InputProps {
        /// The `name` attribute for this form element.
        pub name: AttrValue,
        /// The controlled value of this form element.
        pub value: Option<AttrValue>,
        /// NodeRef referencing the HtmlInputElement
        #[prop_or_default]
        pub input_ref: NodeRef,
        /// The callback to be used for propagating changes to this element's value.
        #[prop_or_default]
        pub update: Callback<InputEvent>,
        #[prop_or_default]
        pub classes: Classes,
        /// The input type of this component.
        #[prop_or(InputType::Text)]
        pub r#type: InputType,
        /// The placeholder value for this component.
        #[prop_or_default]
        pub placeholder: Option<AttrValue>,
        /// Datalist id
        #[prop_or_default]
        pub list: Option<AttrValue>,
        #[prop_or(true)]
        pub autocomplete: bool,
        /// The size of this component.
        #[prop_or_default]
        pub size: Option<Size>,
        /// The color of the border
        #[prop_or_default]
        pub color: Option<Color>,
        /// The background color of the input
        #[prop_or_default]
        pub bg_color: Option<BackgroundColor>,
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
);
//     { def_global_attributes!() },

// }

/// A text input element.
///
/// <https://bulma.io/documentation/form/input/>
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
#[function_component(Input)]
pub fn input(
    InputProps {
        input_ref,
        name,
        value,
        update,
        classes,
        r#type,
        placeholder,
        list,
        autocomplete,
        size,
        rounded,
        loading,
        disabled,
        readonly,
        r#static,
        color,
        bg_color,
        tabindex,
        hidden,
    }: &InputProps,
    // props: &InputProps,
) -> Html {
    let classes = classes!(
        "input",
        classes.clone(),
        size,
        color,
        bg_color,
        rounded.then_some("is-rounded"),
        loading.then_some("is-loading"),
        r#static.then_some("is-static"),
    );

    let autocomplete = if *autocomplete { "on" } else { "off" };
    let r#type: &'static str = r#type.into();

    html! {
        <input
            ref={input_ref}
            {name}
            {value}
            oninput={update}
            class={classes}
            type={r#type}
            {placeholder}
            {list}
            {autocomplete}
            disabled={*disabled}
            readonly={*readonly}
            tabindex={tabindex}
            hidden={*hidden}
        />
    }
}

/// The 4 allowed types for an input component.
///
/// <https://bulma.io/documentation/form/input/>
#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum InputType {
    #[strum(to_string = "text")]
    Text,
    #[strum(to_string = "password")]
    Password,
    #[strum(to_string = "email")]
    Email,
    #[strum(to_string = "tel")]
    Tel,
    #[strum(to_string = "date")]
    Date,
}
