use derive_more::Display;
use yew::{html::IntoPropValue, Classes};
use std::borrow::Cow;

/// Common alignment classes.
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum Alignment {
    #[display(fmt = "left")]
    Left,
    #[display(fmt = "centered")]
    Centered,
    #[display(fmt = "right")]
    Right,
}

/// Common size classes.
#[derive(Clone, Copy, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum Size {
    #[display(fmt = "small")]
    Small,
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
}

impl IntoPropValue<Cow<'static, str>> for Size {
    fn into_prop_value(self) -> Cow<'static, str> {
        Cow::from(self.to_string())
    }
}

impl Into<Classes> for Size {
    fn into(self) -> Classes {
        self.to_string().into()
    }
}