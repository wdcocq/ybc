use yew::events::InputEvent;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct RadioProps {
    /// The `name` attribute for this form element.
    ///
    /// All members of the same radio group must have the same value for their `name` attribute.
    pub name: AttrValue,
    /// The `value` attribute for this form element.
    ///
    /// This is different from other form elements, as this value does not change. It represents
    /// the value to be used for the radio group overall when this element is selected.
    pub value: AttrValue,
    /// The value of the currently selected radio of this radio group.
    pub checked_value: Option<AttrValue>,
    /// The callback to be used for propagating changes to the selected radio of the radio group.
    pub update: Callback<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// The mutually exclusive radio buttons in their native format.
///
/// [https://bulma.io/documentation/form/radio/](https://bulma.io/documentation/form/radio/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
#[function_component(Radio)]
pub fn radio(
    RadioProps {
        name,
        value,
        checked_value,
        update,
        children,
        classes,
        disabled,
    }: &RadioProps,
) -> Html {
    let classes = classes!(classes.clone(), "radio");

    html! {
        <label class={classes} disabled={*disabled}>
            <input
                type="radio"
                {name}
                {value}
                checked={matches!(checked_value, Some(v) if v == value)}
                oninput={update.reform(|e: InputEvent| e.data().unwrap_or_default())}
                disabled={*disabled}
                />
            {children.clone()}
        </label>
    }
}
