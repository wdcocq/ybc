use yew::events::InputEvent;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct RadioProps {
    /// The `name` attribute for this form element.
    ///
    /// All members of the same radio group must have the same value for their `name` attribute.
    pub name: String,
    /// The `value` attribute for this form element.
    ///
    /// This is different from other form elements, as this value does not change. It represents
    /// the value to be used for the radio group overall when this element is selected.
    pub value: String,
    /// The value of the currently selected radio of this radio group.
    pub checked_value: Option<String>,
    /// The callback to be used for propagating changes to the selected radio of the radio group.
    pub update: Callback<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<Classes>,
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
pub struct Radio;

impl Component for Radio {
    type Message = String;
    type Properties = RadioProps;

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
        let mut classes = Classes::from("radio");
        classes.push(&props.classes);
        html! {
            <label class={classes} disabled={props.disabled}>
                <input
                    type="radio"
                    name={props.name.clone()}
                    value={props.value.clone()}
                    checked={props.checked_value.as_ref().map(|val| val == &props.value).unwrap_or(false)}
                    oninput={link.callback(|e: InputEvent| e.data().unwrap_or_default())}
                    disabled={props.disabled}
                    />
                {props.children.clone()}
            </label>
        }
    }
}
