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

impl From<Alignment> for Classes {
    fn from(alignment: Alignment) -> Self {
        alignment.to_string().into()
    }
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

impl From<Size> for Classes {
    fn from(size: Size) -> Self {
        size.to_string().into()
    }
}