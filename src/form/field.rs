use derive_more::Display;
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
    /// Has icons on the left of the field's controls.
    #[prop_or_default]
    pub icons_left: bool,
    /// Has icons on the right of the field's controls.
    #[prop_or_default]
    pub icons_right: bool,
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
}

/// A container for form controls
#[function_component(Field)]
pub fn field(
    FieldProps {
        children,
        classes,
        label,
        label_classes,
        help,
        help_classes,
        help_has_error,
        icons_left,
        icons_right,
        addons,
        addons_align,
        grouped,
        grouped_align,
        multiline,
        horizontal,
    }: &FieldProps,
) -> Html {
    let classes = classes!(
        classes.clone(),
        "field",
        icons_left.then_some("has-icons-left"),
        icons_right.then_some("has-icons-right"),
        addons.then_some("has-addons"),
        addons_align,
        grouped.then_some("is-grouped"),
        grouped_align,
        multiline.then_some("is-grouped-multiline"),
    );

    let label = html! {
        if let Some(label) = label {
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
/// https://bulma.io/documentation/form/general/
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "has-addons-{}")]
pub enum AddonsAlign {
    #[display(fmt = "centered")]
    Centered,
    #[display(fmt = "right")]
    Right,
}

impl From<AddonsAlign> for Classes {
    fn from(align: AddonsAlign) -> Self {
        align.to_string().into()
    }
}

/// The two alignment options available for grouped field controls.
///
/// https://bulma.io/documentation/form/general/
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-grouped-{}")]
pub enum GroupedAlign {
    #[display(fmt = "centered")]
    Centered,
    #[display(fmt = "right")]
    Right,
}

impl From<GroupedAlign> for Classes {
    fn from(align: GroupedAlign) -> Self {
        align.to_string().into()
    }
}

/// The three sizes available for horizontal field labels.
///
/// https://bulma.io/documentation/form/general/#horizontal-form
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum LabelSize {
    #[display(fmt = "small")]
    Small,
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
}

impl From<LabelSize> for Classes {
    fn from(size: LabelSize) -> Self {
        size.to_string().into()
    }
}
