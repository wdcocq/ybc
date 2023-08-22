use strum::IntoStaticStr;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FieldProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
    /// A text label for the field.
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub label_html: Option<Html>,
    /// Extra classes for the label container.
    #[prop_or_default]
    pub label_classes: Classes,
    /// A help message for the field.
    #[prop_or_default]
    pub help: Option<AttrValue>,
    /// Extra classes for the help message container.
    #[prop_or_default]
    pub help_classes: Classes,
    /// A convenience bool to add the `is-danger` class to the help classes when `true`.
    #[prop_or_default]
    pub help_has_error: bool,
    /// Allow addons to the field's controls.
    #[prop_or_default]
    pub addons: bool,
    /// Alignment for the field's addons.
    #[prop_or_default]
    pub addons_align: Option<AddonsAlign>,
    /// All controls in this field should be grouped.
    #[prop_or_default]
    pub grouped: bool,
    /// Alignment for grouped controls.
    #[prop_or_default]
    pub grouped_align: Option<GroupedAlign>,
    /// Allow the grouped controls to span multiple lines.
    #[prop_or_default]
    pub multiline: bool,
    /// Make this a horizontal field.
    #[prop_or_default]
    pub horizontal: bool,
    /// Makes this a narrow field.
    #[prop_or_default]
    pub narrow: bool,
}

/// A container for form controls
#[function_component(Field)]
pub fn field(
    FieldProps {
        children,
        classes,
        label,
        label_html,
        label_classes,
        help,
        help_classes,
        help_has_error,
        addons,
        addons_align,
        grouped,
        grouped_align,
        multiline,
        horizontal,
        narrow,
    }: &FieldProps,
) -> Html {
    let classes = classes!(
        classes.clone(),
        "field",
        horizontal.then_some("is-horizontal"),
        addons.then_some("has-addons"),
        addons_align,
        grouped.then_some("is-grouped"),
        grouped_align,
        multiline.then_some("is-grouped-multiline"),
        narrow.then_some("is-narrow"),
    );

    let label = html! {
        if let Some(label) = label_html.clone().or_else(|| label.as_ref().map(|l| html!{l})) {
            if *horizontal {
                <div class={classes!(label_classes.clone(), "field-label")}>
                    <label class="label">{label}</label>
                </div>
            } else {
                <label class={classes!(label_classes.clone(), "label")}>{label}</label>
            }
        }
    };

    let help = html! {
        if let Some(help) = help {
            <label class={classes!(help_classes.clone(), "help", help_has_error.then_some("is-danger"))}>
               {help}
            </label>
        }
    };

    html! {
        <div class={classes}>
            {label}
            if *horizontal {
                <div class="field-body">{children.clone()}</div>
            } else {
                {children.clone()}
            }
            {help}
        </div>
    }
}

/// The two alignment options available for field addons.
///
/// <https://bulma.io/documentation/form/general/>
#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum AddonsAlign {
    #[strum(to_string = "has-addons-centered")]
    Centered,
    #[strum(to_string = "has-addons-right")]
    Right,
}

/// The two alignment options available for grouped field controls.
///
/// <https://bulma.io/documentation/form/general/>
#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum GroupedAlign {
    #[strum(to_string = "is-grouped-centered")]
    Centered,
    #[strum(to_string = "is-grouped-right")]
    Right,
}

/// The three sizes available for horizontal field labels.
///
/// <https://bulma.io/documentation/form/general/#horizontal-form>
#[derive(Clone, Copy, Debug, IntoStaticStr, PartialEq, Eq)]
pub enum LabelSize {
    #[strum(to_string = "is-small")]
    Small,
    #[strum(to_string = "is-medium")]
    Medium,
    #[strum(to_string = "is-large")]
    Large,
}

impl_classes_from!(AddonsAlign, GroupedAlign, LabelSize);
