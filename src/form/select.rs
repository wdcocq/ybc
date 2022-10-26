#![allow(clippy::redundant_closure_call)]

use web_sys::HtmlSelectElement;
use yew::events::Event;
use yew::prelude::*;

use crate::Size;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SelectProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// NodeRef referencing the HtmlSelectElement
    #[prop_or_default]
    pub r#ref: NodeRef,
    /// The controlled value of this form element.
    #[prop_or_default]
    pub value: Option<String>,
    /// The callback to be used for propagating changes to this element's value.
    #[prop_or_default]
    pub update: Callback<Event>,

    /// The `option` & `optgroup` tags of this select component.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,

    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// A wrapper around an HTML `select` tag.
///
/// [https://bulma.io/documentation/form/select/](https://bulma.io/documentation/form/select/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
///
/// **NOTE WELL:** not all browsers will honor the value of the select element's value on initial
/// load. So if you have an initial `value` set for this component, ensure that the corresponding
/// option element also has the `selected=true` attribute.
#[function_component(Select)]
pub fn select(
    SelectProps {
        name,
        r#ref,
        value,
        update,
        children,
        classes,
        size,
        loading,
        disabled,
    }: &SelectProps,
) -> Html {
    let classes = classes!("select", classes.clone(), size, loading.then_some("is-loading"));

    html! {
        <div class={classes}>
            <select
                ref={r#ref}
                name={name.clone()}
                value={value.clone()}
                disabled={*disabled}
                onchange={update.clone()}
            >
                {children.clone()}
            </select>
        </div>
    }
}

//////////////////////////////////////////////////////////////////////////////

#[derive(Properties, Clone, PartialEq)]
pub struct MultiSelectProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub value: Vec<String>,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<Vec<String>>,

    /// NodeRef referencing the HtmlSelectElement
    #[prop_or_default]
    pub r#ref: NodeRef,
    /// The `option` & `optgroup` tags of this select component.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,

    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Size of the list to display.
    #[prop_or_else(|| 4)]
    pub list_size: u32,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// A wrapper around an HTML `select` tag with the `multiple=true` attribute.
///
/// [https://bulma.io/documentation/form/select/](https://bulma.io/documentation/form/select/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
///
/// **NOTE WELL:** not all browsers will honor the value of the select element's value on initial
/// load. So if you have an initial `value` set for this component, ensure that the corresponding
/// option element also has the `selected=true` attribute.
#[function_component(MultiSelect)]
pub fn multi_select(
    MultiSelectProps {
        name,
        value,
        update,
        r#ref,
        children,
        classes,
        size,
        list_size,
        loading,
        disabled,
    }: &MultiSelectProps,
) -> Html {
    let classes = classes!("select", "is-multiple", classes.clone(), size, loading.then_some("is-loading"));

    let update = update.reform(|e: Event| {
        let select = e.target_unchecked_into::<HtmlSelectElement>();
        let opts = select.selected_options();
        (0..opts.length())
            .into_iter()
            .filter_map(|idx| opts.item(idx))
            .filter_map(|elem| elem.get_attribute("value").or_else(|| elem.text_content()))
            .collect::<Vec<_>>()
    });

    html! {
        <div class={classes}>
            <select
                ref={r#ref}
                multiple={true}
                size={list_size.to_string()}
                name={name.clone()}
                value={value.join(",")}
                disabled={*disabled}
                onchange={update}
            >
                {children.clone()}
            </select>
        </div>
    }
}
