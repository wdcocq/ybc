#![allow(clippy::redundant_closure_call)]

use web_sys::HtmlSelectElement;
use yew::events::Event;
use yew::prelude::*;

use crate::Size;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SelectProps {
    /// The `name` attribute for this form element.
    pub name: String,
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
    pub classes: Option<Classes>,

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
pub struct Select;

impl Component for Select {
    type Message = Event;
    type Properties = SelectProps;

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
        let mut classes = Classes::from("select");
        classes.push(&props.classes);
        if let Some(size) = &props.size {
            classes.push(&size.to_string());
        }
        if props.loading {
            classes.push("is-loading");
        }
        html! {
            <div class={classes}>
                <select
                    name={props.name.clone()}
                    value={props.value.clone()}
                    disabled={props.disabled}
                    onchange={link.callback(|e: Event| e)}
                >
                    {props.children.clone()}
                </select>
            </div>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Properties, Clone, PartialEq)]
pub struct MultiSelectProps {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub value: Vec<String>,
    /// The callback to be used for propagating changes to this element's value.
    pub update: Callback<Vec<String>>,

    /// The `option` & `optgroup` tags of this select component.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,

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
pub struct MultiSelect;

impl Component for MultiSelect {
    type Message = Vec<String>;
    type Properties = MultiSelectProps;

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
        let mut classes = Classes::from("select is-multiple");
        classes.push(&props.classes);
        if let Some(size) = &props.size {
            classes.push(&size.to_string());
        }
        if props.loading {
            classes.push("is-loading");
        }

        let size: String = props.list_size.to_string();
        html! {
            <div class={classes}>
                <select
                    multiple={true}
                    size={size}
                    name={props.name.clone()}
                    value={props.value.join(",")}
                    disabled={props.disabled}
                    onchange={link.callback(|e: Event| {
                        let select: HtmlSelectElement = e.target_unchecked_into();
                        let opts = select.selected_options();
                        (0..opts.length()).into_iter()
                            .filter_map(|idx| opts.item(idx))
                            .filter_map(|elem| elem.get_attribute("value").or_else(|| elem.text_content()))
                            .collect::<Vec<_>>()
                    })}
                >
                    {props.children.clone()}
                </select>
            </div>
        }
    }
}
