use yew::events::InputEvent;
use yew::prelude::*;

use crate::Size;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TextAreaProps {
    /// The `name` attribute for this form element.
    pub name: AttrValue,
    /// The controlled value of this form element.
    #[prop_or_default]
    pub value: Option<AttrValue>,
    /// The callback to be used for propagating changes to this element's value.
    #[prop_or_default]
    pub update: Callback<String>,

    #[prop_or_default]
    pub classes: Classes,
    /// The placeholder value for this component.
    #[prop_or_default]
    pub placeholder: AttrValue,
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
#[function_component(TextArea)]
pub fn text_area(
    TextAreaProps {
        name,
        value,
        update,
        classes,
        placeholder,
        rows,
        size,
        fixed_size,
        loading,
        disabled,
        readonly,
        r#static,
    }: &TextAreaProps,
) -> Html {
    let classes = classes!(
        classes.clone(),
        "textarea",
        size,
        loading.then_some("is-loading"),
        r#static.then_some("is-static"),
        fixed_size.then_some("has-fixed-size")
    );

    html! {
        <textarea
            class={classes}
            {name}
            {value}
            oninput={update.reform(|e: InputEvent| e.data().unwrap_or_default())}
            rows={rows.to_string()}
            {placeholder}
            disabled={*disabled}
            readonly={*readonly}
            />
    }
}
